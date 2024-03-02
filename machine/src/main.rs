#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]

#[macro_use]
extern crate log;
#[macro_use]
mod macros;

mod console;
#[cfg(feature = "dynamic")]
mod dynamic;
#[cfg(feature = "fdt")]
mod fdt;
mod reset;

use core::sync::atomic::{AtomicUsize, Ordering};

const LEN_STACK_PER_HART: usize = 16 * 1024;
pub(crate) const NUM_HART_MAX: usize = 8;
const LEN_STACK: usize = LEN_STACK_PER_HART * NUM_HART_MAX;

static BOOT_LOCK: AtomicUsize = AtomicUsize::new(UNINITIALIZED);
const UNINITIALIZED: usize = 0;
const EARLY_BOOTING: usize = 1;
const FINISHED: usize = 2;

#[cfg(feature = "dynamic")]
static DYNAMIC_INFO: spin::RwLock<Option<dynamic::DynamicInfo>> = spin::RwLock::new(None);

extern "C" fn main(hart_id: usize, opaque: usize, a2: usize) -> usize {
    // TODO the hart clearing the '.bss' segment harts may enter this main function later,
    // causing the variable 'BOOT_LOCK' cleared to zero after the `compare_exchange` here.
    let old_boot_state = match BOOT_LOCK.compare_exchange(
        UNINITIALIZED,
        EARLY_BOOTING,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(s) | Err(s) => s,
    };

    if old_boot_state == UNINITIALIZED {
        console::init();

        trace!("hart {} obtained boot lock", hart_id);
        info!("Early console initialized using UART16550 @ 0x10000000");

        #[cfg(feature = "fdt")]
        if let Ok(fdt) = fdt::try_read_fdt(opaque) {
            let mut board = fdt::FdtBoard::new();
            fdt::parse_fdt(fdt, &mut board);
            board.init();
        }
        #[cfg(not(feature = "fdt"))]
        let _ = opaque;

        info!("RustSBI version {}", rustsbi::VERSION);
        for line in rustsbi::LOGO.lines() {
            info!("{}", line);
        }
        info!("Initializing RustSBI machine-mode environment.");
        BOOT_LOCK.store(FINISHED, Ordering::SeqCst);
    } else {
        while BOOT_LOCK.load(Ordering::SeqCst) != FINISHED {
            core::hint::spin_loop()
        }
    }

    #[cfg(feature = "dynamic")]
    if let Some(mut write) = DYNAMIC_INFO.try_write() {
        trace!(
            "hart {} is reading dynamic info from physical address 0x{:x}",
            hart_id,
            a2
        );
        if let Ok(info) = dynamic::try_read_dynamic(a2) {
            trace!(
                "dynamic info magic: {:x}, version: {}",
                info.magic,
                info.version
            );
            // TODO check magic and version
            trace!(
                "dynamic info would like to jump to address 0x{:x} with mode {}",
                info.next_addr,
                info.next_mode
            );
            // TODO options (we don't use it by now)
            trace!("dynamic info has extra option: {:x}", info.options);
            info!("Redirecting harts to address 0x{:x}", info.next_addr);
            *write = Some(info);
        } else {
            error!("read dynamic info failed");
            reset::fail()
        }
    }
    #[cfg(not(feature = "dynamic"))]
    let _ = a2;

    match () {
        #[cfg(feature = "dynamic")]
        () => loop {
            if let Some(info) = *DYNAMIC_INFO.read() {
                return info.next_addr;
            }
            core::hint::spin_loop()
        },
        // TODO non-dynamic supervisor address
        #[cfg(not(feature = "dynamic"))]
        () => {
            static NON_DYNAMIC_LOG_ONCE: spin::Once<()> = spin::Once::new();
            NON_DYNAMIC_LOG_ONCE.call_once(|| {
                error!("non-dynamic jump address is not yet supported");
                reset::fail()
            });
            trace!("wait before shutdown");
            loop {
                core::hint::spin_loop()
            }
        }
    }
}

// TODO contribute `Stack` struct into the crate `riscv`
#[repr(C, align(128))]
struct Stack<const N: usize>([u8; N]);

#[link_section = ".bss.uninit"]
static STACK: Stack<LEN_STACK> = Stack([0; LEN_STACK]);

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
    error!("panicked at {}", info);
    loop {}
}
