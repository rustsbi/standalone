use crate::Config;
use os_xtask_utils::{BinUtil, Cargo, CommandExt};

const TARGET: &'static str = "riscv64imac-unknown-none-elf";

pub fn build_no_specific_platform(config: &Config) {
    let features = machine_features_from_config(config);
    Cargo::build()
        .package("rustsbi-machine")
        .features(false, features)
        .target(TARGET)
        .release()
        .invoke();
    let elf_path = crate::PROJECT
        .join("target")
        .join(TARGET)
        .join("release")
        .join("rustsbi-machine");
    let bin_path = elf_path.with_extension("bin");
    BinUtil::objcopy()
        .arg("--binary-architecture=riscv64")
        .arg(elf_path)
        .args(["--strip-all", "-O", "binary"])
        .arg(&bin_path)
        .invoke();
}

fn machine_features_from_config(config: &Config) -> Vec<&'static str> {
    let mut ans = Vec::new();
    if config.machine_fdt_ident_enabled.unwrap_or(true) {
        ans.push("fdt");
    }
    ans
}
