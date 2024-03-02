// Ref: https://github.com/rustsbi/rustsbi-d1/tree/main/xtask

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{ErrorKind, Seek, SeekFrom},
    path::Path,
};

use crate::{app::Bootstrap, tool::Xfel, Config};
use log::error;
use os_xtask_utils::{BinUtil, Cargo, CommandExt};

const TARGET: &'static str = "riscv64imac-unknown-none-elf";

pub fn build_allwinner_d1_series(config: &Config) {
    let features = bootstrap_features_from_config(config);
    Cargo::build()
        .package("rustsbi-bootstrap")
        .features(false, features)
        .target(TARGET)
        .release()
        .invoke();
    let elf_path = crate::PROJECT
        .join("target")
        .join(TARGET)
        .join("release")
        .join("rustsbi-bootstrap");
    let bin_path = elf_path.with_extension("bin");
    BinUtil::objcopy()
        .arg("--binary-architecture=riscv64")
        .arg(elf_path)
        .args(["--strip-all", "-O", "binary"])
        .arg(&bin_path)
        .invoke();
    xtask_finialize_d1_flash_bt0(&bin_path);
}

pub fn flash_allwinner_d1_series() {
    let bin_path = crate::PROJECT
        .join("target")
        .join(TARGET)
        .join("release")
        .join("rustsbi-bootstrap.bin");
    Xfel::spinand_write(0, &bin_path).invoke();
}

fn bootstrap_features_from_config(config: &Config) -> Vec<&'static str> {
    match config.bootstrap {
        Bootstrap::JumpToDram => vec!["jump-to-dram"],
        Bootstrap::HelloWorld => vec!["sample-hello-world"],
        Bootstrap::SpiFlash => vec!["sample-spi-flash"],
        Bootstrap::NoBootstrap => todo!(),
    }
}

// Ref: https://github.com/luojia65/test-d1-flash-bare/blob/main/xtask/src/main.rs

const EGON_HEADER_LENGTH: u64 = 0x60;

// This function does:
// 1. fill in binary length
// 2. calculate checksum of bt0 image; old checksum value must be filled as stamp value
fn xtask_finialize_d1_flash_bt0(bin_path: &Path) {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(bin_path)
        .expect("open output binary file");
    let total_length = file.metadata().unwrap().len();
    if total_length < EGON_HEADER_LENGTH {
        error!(
            "objcopy binary size less than eGON header length, expected >= {} but is {}",
            EGON_HEADER_LENGTH, total_length
        );
    }
    let new_len = align_up_to(total_length, 16 * 1024); // align up to 16KB
    file.set_len(new_len).unwrap();
    file.seek(SeekFrom::Start(0x10)).unwrap();
    file.write_u32::<LittleEndian>(new_len as u32).unwrap();
    file.seek(SeekFrom::Start(0x0C)).unwrap();
    let stamp = file.read_u32::<LittleEndian>().unwrap();
    if stamp != 0x5F0A6C39 {
        error!("wrong stamp value; check your generated blob and try again")
    }
    let mut checksum: u32 = 0;
    file.seek(SeekFrom::Start(0)).unwrap();
    loop {
        match file.read_u32::<LittleEndian>() {
            Ok(val) => checksum = checksum.wrapping_add(val),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => error!("io error while calculating checksum: {:?}", e),
        }
    }
    file.seek(SeekFrom::Start(0x0C)).unwrap();
    file.write_u32::<LittleEndian>(checksum).unwrap();
    file.sync_all().unwrap(); // save file before automatic closing
} // for C developers: files are automatically closed when they're out of scope

fn align_up_to(len: u64, target_align: u64) -> u64 {
    let (div, rem) = (len / target_align, len % target_align);
    if rem != 0 {
        (div + 1) * target_align
    } else {
        len
    }
}
