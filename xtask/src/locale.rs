use once_cell::sync::Lazy;
use std::collections::HashMap;

#[rustfmt::skip]
static LOCALE: Lazy<HashMap<&'static str, HashMap<&'static str, &'static str>>> = Lazy::new(|| HashMap::from([
    ("id", [("zh-CN", "编号"), ("en-US", "Identifier")].into()),
    ("back", [("zh-CN", "返回"), ("en-US", "Back")].into()),
    ("home.title", [("zh-CN", " RustSBI 原型设计系统 - 主界面 "), ("en-US", " RustSBI Prototyping System - Home page ")].into()),
    ("home.language", [("zh-CN", "配置语言"), ("en-US", "Language settings")].into()),
    ("home.bootstrap", [("zh-CN", "启动程序"), ("en-US", "Bootstrap program")].into()),
    ("home.item", [("zh-CN", "选项"), ("en-US", "Item")].into()),
    ("home.brief", [("zh-CN", "简述"), ("en-US", "Brief")].into()),
    ("home.machine-mode", [("zh-CN", "机器态功能"), ("en-US", "Machine mode")].into()),
    ("home.supervisor-mode", [("zh-CN", "内核态功能"), ("en-US", "Supevisor mode")].into()),
    ("home.platform-support", [("zh-CN", "平台支持"), ("en-US", "Platform support")].into()),
    ("home.bootload-media", [("zh-CN", "引导介质"), ("en-US", "Bootloading media")].into()),
    ("home.compile-flags", [("zh-CN", "编译配置"), ("en-US", "Compile flags")].into()),
    ("home.help-ver-about", [("zh-CN", "帮助关于"), ("en-US", "Help, version & about")].into()),
    ("home.quit-and-save", [("zh-CN", "退出并保存"), ("en-US", "Quit and save")].into()),
    ("language.display.current", [("zh-CN", "简体中文（中国）"), ("en-US", "English (US)")].into()),
    ("language.display.zh-CN", [("zh-CN", "简体中文（中国）"), ("en-US", "Simplified Chinese (China)")].into()),
    ("language.display.en-US", [("zh-CN", "英文（美国）"), ("en-US", "English (US)")].into()),
    ("language.title", [("zh-CN", " RustSBI 原型设计系统 - 语言选项 "), ("en-US", " RustSBI Prototyping System - Language settings ")].into()),
    ("language.language", [("zh-CN", "语言"), ("en-US", "Language")].into()),
    ("bootstrap.title", [("zh-CN", " RustSBI 原型设计系统 - 启动程序 "), ("en-US", " RustSBI Prototyping System - Bootstrap program ")].into()),
    ("bootstrap.jump-to-dram", [("zh-CN", "跳转至 DRAM"), ("en-US", "Jump to DRAM")].into()),
    ("bootstrap.sample-program", [("zh-CN", "仅启动示例程序"), ("en-US", "Start sample program only")].into()),
    ("sample-program.title", [("zh-CN", " RustSBI 原型设计系统 - 示例程序 "), ("en-US", " RustSBI Prototyping System - Sample program ")].into()),
    ("sample-program.chosen", [("zh-CN", "已选中"), ("en-US", "Chosen")].into()),
    ("sample-program.not-chosen", [("zh-CN", "未选中"), ("en-US", "Not chosen")].into()),
    ("sample-program.platform-not-supported", [("zh-CN", "目标平台不支持此程序"), ("en-US", "Target does not support this program")].into()),
    ("sample-program.hello-world", [("zh-CN", "Hello World 示例程序"), ("en-US", "Hello World sample program")].into()),
    ("sample-program.spi-flash", [("zh-CN", "SPI 闪存示例程序"), ("en-US", "SPI flash sample program")].into()),
    ("sample-program.not-sample-program", [("zh-CN", "不使用示例程序"), ("en-US", "Not using sample programs")].into()),
    ("machine-mode.title", [("zh-CN", " RustSBI 原型设计系统 - 机器态功能 "), ("en-US", " RustSBI Prototyping System - Machine mode features ")].into()),
    ("machine-mode.standard-sbi", [("zh-CN", "标准 SBI 功能"), ("en-US", "Standard SBI features")].into()),
    ("machine-mode.not-supported", [("zh-CN", "启动程序不支持机器态功能"), ("en-US", "Bootstrap program does not support machine mode features")].into()),
    ("platform-support.title", [("zh-CN", " RustSBI 原型设计系统 - 平台支持 "), ("en-US", " RustSBI Prototyping System - Platform support ")].into()),
    ("platform-support.allwinner-d1-series", [("zh-CN", "全志® D1-H 系列平台"), ("en-US", "Allwinner® D1-H series")].into()),
    ("platform-support.choose-platform", [("zh-CN", "选择此平台"), ("en-US", "Choose this platform")].into()),
    ("platform-support.chosen", [("zh-CN", "已选中此平台"), ("en-US", "Platform chosen")].into()),
    ("platform-support.not-chosen", [("zh-CN", "未选中此平台"), ("en-US", "Platform not chosen")].into()),
    ("platform-support.no-platform-chosen", [("zh-CN", "未选中任何平台"), ("en-US", "No platform chosen")].into()),
    ("allwinner-d1-series.title", [("zh-CN", " RustSBI 原型设计系统 - 全志® D1-H 系列平台 "), ("en-US", " RustSBI Prototyping System - Allwinner® D1-H series ")].into()),
    ("standard-sbi-features.title", [("zh-CN", " RustSBI 原型设计系统 - 标准 SBI 功能 "), ("en-US", " RustSBI Prototyping System - Standard SBI features ")].into()),
    ("standard-sbi-features.timer", [("zh-CN", "时钟扩展"), ("en-US", "Timer extension")].into()),
    ("standard-sbi-features.ipi", [("zh-CN", "核间中断扩展"), ("en-US", "Inter-processor interrupt extension")].into()),
    ("standard-sbi-features.rfence", [("zh-CN", "远程栅栏扩展"), ("en-US", "Remote fence extension")].into()),
    ("standard-sbi-features.hsm", [("zh-CN", "核状态扩展"), ("en-US", "Hart state monitor extension")].into()),
    ("standard-sbi-features.srst", [("zh-CN", "系统复位扩展"), ("en-US", "System reset extension")].into()),
    ("standard-sbi-features.pmu", [("zh-CN", "性能监视扩展"), ("en-US", "Performance monitor extension")].into()),
    ("standard-sbi-features.v1p0-prepared", [("zh-CN", "标准 SBI 1.0 实现"), ("en-US", "Standard SBI 1.0 implementation")].into()),
    ("standard-sbi-features.partial", [("zh-CN", "仅启用部分 SBI 扩展"), ("en-US", "Parital SBI extension(s) enabled")].into()),
    ("standard-sbi-features.no-support", [("zh-CN", "不支持 SBI 功能"), ("en-US", "No SBI features supported")].into()),
    ("standard-sbi-features.enabled", [("zh-CN", "已启用"), ("en-US", "Enabled")].into()),
    ("standard-sbi-features.disabled", [("zh-CN", "已禁用"), ("en-US", "Disabled")].into()),
]));

pub fn get_string(idx: &str, locale: &str) -> &'static str {
    match LOCALE.get(idx) {
        Some(table) => match table.get(locale) {
            Some(ans) => ans,
            None => "",
        },
        None => "",
    }
}

pub trait Translate {
    fn translate(self, locale: &str) -> Self;
}

impl<const N: usize> Translate for [&'static str; N] {
    fn translate(self, locale: &str) -> Self {
        let mut ans = [""; N];
        for i in 0..N {
            ans[i] = get_string(self[i], locale);
        }
        ans
    }
}
impl Translate for Vec<&'static str> {
    fn translate(self, locale: &str) -> Self {
        let mut ans = vec![""; self.len()];
        for i in 0..self.len() {
            ans[i] = get_string(self[i], locale);
        }
        ans
    }
}

impl Translate for &str {
    fn translate(self, locale: &str) -> Self {
        get_string(&self, locale)
    }
}
