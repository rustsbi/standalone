use std::{env, path::PathBuf};

fn main() {
    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("rom-rt.ld");
    #[cfg(feature = "allwinner-d1")]
    std::fs::write(ld, LINKER_ALLWINNER_D1).unwrap();
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}

#[cfg_attr(not(feature = "allwinner-d1"), allow(unused))]
const LINKER_ALLWINNER_D1: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(head_jump)
MEMORY {
    SRAM : ORIGIN = 0x00020000, LENGTH = 32K
}
SECTIONS {
    .head : {
        KEEP(*(.head.text))
        KEEP(*(.head.egon))
        KEEP(*(.head.jump))
        . = ALIGN(4);
        shmeta = .;
        KEEP(*(.head.meta))
        . = ALIGN(128);
        KEEP(*(.magic.tail))
        KEEP(*(.magic.head))
    } > SRAM
    .text : ALIGN(4) {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > SRAM
    .rodata : ALIGN(8) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);
        erodata = .;
    } > SRAM
    .data : ALIGN(8) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8);
        edata = .;
    } > SRAM
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(8) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";