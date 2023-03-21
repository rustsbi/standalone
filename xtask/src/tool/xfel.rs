// Source: https://github.com/rustsbi/rustsbi-d1/blob/main/xtask/src/main.rs
#![allow(unused)] // FIXME: remove when use

use log::{error, info};
use once_cell::sync::Lazy;
use os_xtask_utils::{ext, CommandExt, Ext};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

ext!(def; Xfel);

static PATH: Lazy<PathBuf> = Lazy::new(detect_xfel);

impl Xfel {
    #[inline]
    fn new<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut xfel = Command::new(&*PATH);
        xfel.args(args);
        Self(xfel)
    }

    #[inline]
    pub fn write(address: usize, file: impl AsRef<Path>) -> Self {
        let mut ans = Self::new(["write"]);
        ans.arg(format!("{address:#x}")).arg(file.as_ref());
        ans
    }

    #[inline]
    pub fn exec(address: usize) -> Self {
        let mut ans = Self::new(["exec"]);
        ans.arg(format!("{address:#x}"));
        ans
    }

    #[inline]
    pub fn ddr(ty: &str) -> Self {
        Self::new(["ddr", ty])
    }

    #[inline]
    pub fn reset() -> Self {
        Self::new(["reset"])
    }

    #[inline]
    pub fn spinand_read(address: usize, length: usize, file: impl AsRef<Path>) -> Self {
        let mut ans = Self::new(["spinand"]);
        ans.arg("read")
            .arg(format!("{address:#x}"))
            .arg(format!("{length:#x}"))
            .arg(file.as_ref());
        ans
    }

    #[inline]
    pub fn spinand_write(address: usize, file: impl AsRef<Path>) -> Self {
        let mut ans = Self::new(["spinand"]);
        ans.arg("write")
            .arg(format!("{address:#x}"))
            .arg(file.as_ref());
        ans
    }
}

fn detect_xfel() -> PathBuf {
    match Ext::new("xfel").as_mut().output() {
        Ok(output) => {
            if output.status.success() {
                let x = output
                    .stdout
                    .iter()
                    .copied()
                    .skip_while(|c| *c != b'(')
                    .skip(1)
                    .take_while(|c| *c != b')')
                    .collect::<Vec<u8>>();
                info!(
                    "detected xfel of version {:?}",
                    std::str::from_utf8(&x).unwrap()
                );
                PathBuf::from("xfel")
            } else {
                todo!()
            }
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                error!("xfel not found");
                std::process::exit(1);
            } else {
                panic!("{e:?}");
            }
        }
    }
}
