#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]

use core::sync::atomic::{AtomicBool, Ordering::AcqRel};
use rustsbi_standalone::{print, Operation};
use spin::Once;

pub(crate) const LEN_STACK_PER_HART: usize = 16 * 1024;
pub(crate) const NUM_HART_MAX: usize = 8;
pub(crate) const LEN_STACK_SBI: usize = LEN_STACK_PER_HART * NUM_HART_MAX;

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    #[link_section = ".bss.uninit"]
    static mut SBI_STACK: [u8; LEN_STACK_SBI] = [0; LEN_STACK_SBI];

    core::arch::asm!(
        // 1. Turn off interrupt
        "csrw  mie, zero",
        // 2. Initialize programming langauge runtime
        // only clear bss if hartid is zero
        "csrr  t0, mhartid",
        "bnez  t0, 2f",
        // clear bss segment
        "la  t0, sbss",
        "la  t1, ebss",
        "1:",
        "bgeu  t0, t1, 2f",
        "sd  zero, 0(t0)",
        "addi  t0, t0, 8",
        "j  1b",
        "2:",
        // 3. Prepare stack for each hart
        "la  sp, {stack}",
        "li  t0, {per_hart_stack_size}",
        "csrr  t1, mhartid",
        "addi  t1, t1, 1",
        "1: ",
        "add  sp, sp, t0",
        "addi  t1, t1, -1",
        "bnez  t1, 1b",
        "j  {rust_main}",
        // 4. Clean up
        "j  {finalize}",
        per_hart_stack_size = const LEN_STACK_PER_HART,
        stack = sym SBI_STACK,
        rust_main = sym rust_main,
        finalize = sym finalize,
        options(noreturn)
    )
}

/// rust 入口。
extern "C" fn rust_main(_hartid: usize, opaque: usize) -> Operation {
    #[link_section = ".bss.uninit"] // 以免清零
    static INIT_HART: AtomicBool = AtomicBool::new(false);

    // static SERIAL: Once<ns16550a::Ns16550a> = Once::new();
    // static BOARD_INFO: Once<BoardInfo> = Once::new();
    static CSR_PRINT: AtomicBool = AtomicBool::new(false);

    // 全局初始化过程
    if !INIT_HART.swap(true, AcqRel) {
        // 解析设备树
        // let board_info = BOARD_INFO.call_once(|| device_tree::parse(opaque));
        // let hsm = HSM.call_once(|| qemu_hsm::QemuHsm::new(NUM_HART_MAX, opaque));
        // 打印启动信息
        //         print!(
        //             "\
        // [rustsbi] RustSBI version {ver_sbi}, adapting to RISC-V SBI v1.0.0
        // {logo}
        // [rustsbi] Implementation     : RustSBI-QEMU Version {ver_impl}
        // [rustsbi] Platform Name      : {model}
        // [rustsbi] Platform SMP       : {smp}
        // [rustsbi] Platform Memory    : {mem:#x?}
        // [rustsbi] Boot HART          : {hartid}
        // [rustsbi] Device Tree Region : {dtb:#x?}
        // [rustsbi] Firmware Address   : {firmware:#x}
        // [rustsbi] Supervisor Address : {SUPERVISOR_ENTRY:#x}
        // ",
        //             ver_sbi = rustsbi::VERSION,
        //             logo = rustsbi::logo(),
        //             ver_impl = env!("CARGO_PKG_VERSION"),
        //             model = board_info.model,
        //             smp = board_info.smp,
        //             mem = board_info.mem,
        //             hartid = riscv::register::mhartid::read(),
        //             dtb = board_info.dtb,
        //             firmware = entry as usize,
        //         );
    }

    // let hsm = HSM.wait();
    // if let Some(supervisor) = hsm.take_supervisor() {
    //     // use execute::*;
    //     // 设置并打印 pmp
    //     // set_pmp(BOARD_INFO.wait());
    //     if !CSR_PRINT.swap(true, AcqRel) {
    //         // hart_csr_utils::print_pmps();
    //     }
    //     // execute_supervisor(hsm, supervisor)
    // } else {
    //     Operation::Stop
    // }
    todo!()
}

/// 准备好不可恢复休眠或关闭
///
/// 在隔离的环境（汇编）调用，以确保 main 中使用的堆资源完全释放。
/// （只是作为示例，因为这个版本完全不使用堆）
unsafe extern "C" fn finalize(op: Operation) -> ! {
    match op {
        Operation::Stop => {
            // HSM.wait().finalize_before_stop();
            riscv::interrupt::enable();
            // 从中断响应直接回 entry
            loop {
                riscv::asm::wfi();
            }
        }
        Operation::SystemReset => {
            // TODO 等待其他核关闭
            // 直接回 entry
            entry()
        }
    }
}
