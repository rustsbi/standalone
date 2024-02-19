#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
use crate::board::Board;
use core::arch::asm;
use rcore_console::println;

mod board;
mod console;
mod fdt;
mod opaque;

extern "C" fn main(hart_id: usize, opaque: usize) {
    let mut board = Board::new();
    if opaque::is_null(opaque) {
        // nothing to do now ...
        // TODO fixed base address
    }
    // TODO #[cfg(feature = "fdt")]
    if let Ok(fdt) = fdt::try_read_fdt(opaque) {
        fdt::parse_fdt(fdt, &mut board);
    }

    if let Some(serial) = &board.uart16550_serial() {
        console::load_console(serial);
    }

    println!("Hello World!");

    let _ = hart_id; // TODO

    // TODO
}

// TODO contribute `Stack` struct into the crate `riscv`
#[repr(C, align(128))]
struct Stack<const N: usize>([u8; N]);

const LEN_STACK: usize = 1 * 1024;

#[link_section = ".bss.uninit"]
static STACK: Stack<LEN_STACK> = Stack([0; LEN_STACK]);

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn start() {
    asm!(
        "   la      sp, {stack}
            li      t0, {hart_stack_size}
            csrr    t1, mhartid
            addi    t1, t1,  1
        1:  add     sp, sp, t0
            addi    t1, t1, -1
            bnez    t1, 1b",
        "   la      t2, sbss
            la      t3, ebss
        1:  bgeu    t2, t3, 1f
            sd      zero, 0(t2)
            addi    t2, t2, 8
            j       1b
        1:",
        "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 1f
            ld      t6, 0(t3)
            sd      t6, 0(t4)
            addi    t3, t3, 8
            addi    t4, t4, 8
            j       1b
        1:",
        "   j       {main}",
        stack = sym STACK,
        hart_stack_size = const LEN_STACK,
        main = sym main,
        options(noreturn)
    )
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
