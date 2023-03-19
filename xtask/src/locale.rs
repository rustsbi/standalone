use once_cell::sync::Lazy;
use std::collections::HashMap;

#[rustfmt::skip]
static LOCALE: Lazy<HashMap<&'static str, HashMap<&'static str, &'static str>>> = Lazy::new(|| HashMap::from([
    ("id", [("zh-CN", "编号"), ("en-US", "Identifier")].into()),
    ("back", [("zh-CN", "返回"), ("en-US", "Back")].into()),
    ("home.title", [("zh-CN", " RustSBI 原型设计系统 - 主界面 "), ("en-US", " RustSBI Prototyping System - Home page ")].into()),
    ("home.language", [("zh-CN", "配置语言"), ("en-US", "Language settings")].into()),
    ("home.item", [("zh-CN", "选项"), ("en-US", "Item")].into()),
    ("home.brief", [("zh-CN", "简述"), ("en-US", "Brief")].into()),
    ("home.machine-mode", [("zh-CN", "机器态功能"), ("en-US", "Machine mode")].into()),
    ("home.supervisor-mode", [("zh-CN", "内核态功能"), ("en-US", "Supevisor mode")].into()),
    ("home.platform-support", [("zh-CN", "平台支持"), ("en-US", "Platform support")].into()),
    ("home.bootload-media", [("zh-CN", "引导介质"), ("en-US", "Bootloading media")].into()),
    ("home.compile-flags", [("zh-CN", "编译配置"), ("en-US", "Compile flags")].into()),
    ("home.help-ver-about", [("zh-CN", "帮助关于"), ("en-US", "Help, version & about")].into()),
    ("home.quit", [("zh-CN", "退出程序"), ("en-US", "Exit program")].into()),
    ("language.display.current", [("zh-CN", "简体中文（中国）"), ("en-US", "English (US)")].into()),
    ("language.display.zh-CN", [("zh-CN", "简体中文（中国）"), ("en-US", "Simplified Chinese (China)")].into()),
    ("language.display.en-US", [("zh-CN", "英文（美国）"), ("en-US", "English (US)")].into()),
    ("language.title", [("zh-CN", " RustSBI 原型设计系统 - 语言选项 "), ("en-US", " RustSBI Prototyping System - Language settings ")].into()),
    ("language.language", [("zh-CN", "语言"), ("en-US", "Language")].into()),
    ("machine-mode.title", [("zh-CN", " RustSBI 原型设计系统 - 机器态功能 "), ("en-US", " RustSBI Prototyping System - Machine mode features ")].into()),
    ("machine-mode.standard-sbi", [("zh-CN", "标准 SBI 功能"), ("en-US", "Standard SBI features")].into()),
    ("platform-support.title", [("zh-CN", " RustSBI 原型设计系统 - 平台支持 "), ("en-US", " RustSBI Prototyping System - Platform support ")].into()),
    ("platform-support.allwinner-d1-series", [("zh-CN", "全志® D1-H 系列平台"), ("en-US", "Allwinner® D1-H series")].into()),
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
