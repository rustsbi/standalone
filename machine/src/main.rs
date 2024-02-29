#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

#[macro_use]
extern crate log;

mod console;
mod dynamic;
#[cfg(feature = "fdt")]
mod fdt;
mod opaque;

use core::sync::atomic::{Ordering, AtomicBool};

static BOOT_FINISHED: AtomicBool = AtomicBool::new(false);
static BOOT_LOCK: spin::Mutex<()> = spin::Mutex::new(());
#[cfg(feature = "dynamic")]
static DYNAMIC_INFO: spin::RwLock<Option<dynamic::DynamicInfo>> = spin::RwLock::new(None);

extern "C" fn main(hart_id: usize, opaque: usize, a2: usize) -> usize {
    if opaque::is_null(opaque) {
        // nothing to do now ...
        // TODO fixed base address
    }

    if let Some(_) = BOOT_LOCK.try_lock() {
        rcore_console::init_console(&console::RCoreConsole);
        rcore_console::set_log_level(option_env!("LOG"));

        trace!("hart {} obtained boot lock", hart_id);
        info!("Early console initialized using UART16550 @ 0x10000000");
    
        #[cfg(feature = "fdt")]
        if let Ok(fdt) = fdt::try_read_fdt(opaque) {
            let mut board = fdt::FdtBoard::new();
            fdt::parse_fdt(fdt, &mut board);
            board.load_main_console();
        }

        BOOT_FINISHED.store(true, Ordering::Relaxed);
        info!("Starting RustSBI machine-mode environment.");
    } else {
        while !BOOT_FINISHED.load(Ordering::Relaxed) {
            core::hint::spin_loop()
        }
    }

    #[cfg(feature = "dynamic")]
    if let Some(mut write) = DYNAMIC_INFO.try_write() {
        trace!("hart {} is reading dynamic info from physical address 0x{:x}", hart_id, a2);
        if let Ok(dynamic_info) = dynamic::try_read_dynamic(a2) {
            trace!("dynamic info magic: {:x}, version: {}", dynamic_info.magic, dynamic_info.version);
            // TODO check magic and version
            trace!("dynamic info would like to jump to address 0x{:x} with mode {}", dynamic_info.next_addr, dynamic_info.next_mode);
            // TODO options (we don't use it by now)
            trace!("dynamic info has extra option: {:x}", dynamic_info.options);
            *write = Some(*dynamic_info);
        } else {
            debug!("read dynamic info failed");
            // TODO shutdown if applicable
        }
    }

    match () {
        #[cfg(feature = "dynamic")]
        () => {
            loop {
                if let Some(info) = *DYNAMIC_INFO.read() {
                    info!("Redirecting hart {} to address 0x{:x}", hart_id, info.next_addr);
                    return info.next_addr
                }
                core::hint::spin_loop()
            }
        },
        // TODO non-dynamic supervisor address
        #[cfg(not(feature = "dynamic"))]
        () => {
            debug!("non-dynamic jump address is not yet supported");
            // TODO shutdown if applicable
            loop {}
        },
    }
}

// TODO contribute `Stack` struct into the crate `riscv`
#[repr(C, align(128))]
struct Stack<const N: usize>([u8; N]);

#[link_section = ".bss.uninit"]
static STACK: Stack<LEN_STACK> = Stack([0; LEN_STACK]);

pub(crate) const LEN_STACK_PER_HART: usize = 16 * 1024;
pub(crate) const NUM_HART_MAX: usize = 8;
pub(crate) const LEN_STACK: usize = LEN_STACK_PER_HART * NUM_HART_MAX;

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    core::arch::asm!(
        // 1. Turn off interrupt
        "   csrw    mie, zero",
        // 2. Initialize programming langauge runtime
        // only clear bss if hartid is zero
        "   csrr    t0, mhartid",
        "   bnez    t0, 2f",
        // clear bss segment
        "   la      t0, sbss
            la      t1, ebss
        1:  bgeu    t0, t1, 2f
            sd      zero, 0(t0)
            addi    t0, t0, 8
            j       1b",
        // prepare data segment
        "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 2f
            ld      t6, 0(t3)
            sd      t6, 0(t4)
            addi    t3, t3, 8
            addi    t4, t4, 8
            j       1b",
        "2: ",
        // 3. Prepare stack for each hart
        "   la      sp, {stack}",
        "   li      t0, {per_hart_stack_size}",
        "   csrr    t1, mhartid",
        "   addi    t1, t1, 1",
        "1: ",
        "   add     sp, sp, t0",
        "   addi    t1, t1, -1",
        "   bnez    t1, 1b",
        // 4. Run Rust main function
        "   j       {main}",
        // 5. Jump to following boot sequences
        "   jr      a0", // TODO 
        per_hart_stack_size = const LEN_STACK_PER_HART,
        stack = sym STACK,
        main = sym main,
        options(noreturn)
    )
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("panicked at {:?}", info);
    loop {}
}
