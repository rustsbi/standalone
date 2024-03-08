use crate::locale;
use core::mem;
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;
use tui::widgets::TableState;

pub struct App {
    current_navigation: Route,
    navigation_stack: Vec<Route>,
    pub(crate) item_length: usize,
    pub(crate) control_flow_fn: Option<fn(usize, &mut App) -> ControlFlow<(), ()>>,
    pub locale: String,
    pub bootstrap: Bootstrap,
    pub standard_sbi_enabled: StandardSbiEnabled,
    pub machine_mode_fdt_ident_enabled: bool,
    pub platform: Platform,
    pub supervisor_mode_brief: &'static str,
    pub bootload_media_brief: &'static str,
    pub compile_flags_brief: &'static str,
    pub help_ver_about_brief: &'static str,
}

impl App {
    pub fn current_route_mut(&mut self) -> &mut Route {
        &mut self.current_navigation
    }

    pub fn current_route(&self) -> &Route {
        &self.current_navigation
    }

    pub fn push_route(&mut self, id: RouteId) {
        let mut new_route = Route::from_route_id(id);
        mem::swap(&mut new_route, &mut self.current_navigation);
        self.navigation_stack.push(new_route);
    }

    pub fn pop_route(&mut self) -> Option<Route> {
        if let Some(mut route) = self.navigation_stack.pop() {
            mem::swap(&mut self.current_navigation, &mut route);
            Some(route)
        } else {
            None
        }
    }

    pub fn next(&mut self) {
        let state = &mut self.current_navigation.table_state;
        let i = match state.selected() {
            Some(i) => {
                if i >= self.item_length - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let state = &mut self.current_navigation.table_state;
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    self.item_length - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn enter(&mut self) -> ControlFlow<(), ()> {
        let state = &self.current_navigation.table_state;
        match state.selected() {
            Some(idx) => match self.control_flow_fn {
                Some(f) => f(idx, self),
                None => ControlFlow::Continue(()),
            },
            None => ControlFlow::Continue(()),
        }
    }
}

impl App {
    pub fn language_brief(&self) -> &'static str {
        locale::get_string("language.display.current", &self.locale)
    }
    pub fn bootstrap_brief(&self) -> &'static str {
        // when we have other programs, change this function
        match self.bootstrap {
            Bootstrap::JumpToDram => locale::get_string("bootstrap.jump-to-dram", &self.locale),
            Bootstrap::NoBootstrap => locale::get_string("bootstrap.no-bootstrap", &self.locale),
            _ => self.bootstrap_sample_program_brief(),
        }
    }
    pub fn bootstrap_jump_to_dram_brief(&self) -> &'static str {
        let idx = if self.platform.is_bootstrap_supported(&Bootstrap::JumpToDram) {
            if self.bootstrap == Bootstrap::JumpToDram {
                "sample-program.chosen"
            } else {
                "sample-program.not-chosen"
            }
        } else {
            "sample-program.platform-not-supported"
        };
        locale::get_string(idx, &self.locale)
    }
    pub fn bootstrap_sample_program_brief(&self) -> &'static str {
        let idx = match self.bootstrap {
            Bootstrap::HelloWorld => "sample-program.hello-world",
            Bootstrap::SpiFlash => "sample-program.spi-flash",
            _ => "sample-program.not-sample-program",
        };
        locale::get_string(idx, &self.locale)
    }
    pub fn bootstrap_no_bootstrap_brief(&self) -> &'static str {
        let idx = if self
            .platform
            .is_bootstrap_supported(&Bootstrap::NoBootstrap)
        {
            if self.bootstrap == Bootstrap::NoBootstrap {
                "sample-program.chosen"
            } else {
                "sample-program.not-chosen"
            }
        } else {
            "sample-program.platform-not-supported"
        };
        locale::get_string(idx, &self.locale)
    }
    pub fn standard_sbi_brief(&self) -> &'static str {
        let idx = if self.standard_sbi_enabled.sbi_v1p0_ready() {
            "standard-sbi-features.v1p0-prepared"
        } else if self.standard_sbi_enabled.no_sbi_support() {
            "standard-sbi-features.no-support"
        } else {
            "standard-sbi-features.partial"
        };
        locale::get_string(idx, &self.locale)
    }
    pub fn machine_mode_brief(&self) -> &'static str {
        if !self.bootstrap.is_machine_mode_supported() {
            let idx = "machine-mode.not-supported";
            locale::get_string(idx, &self.locale)
        } else {
            // in future when we have custom sbi features here, change this function
            self.standard_sbi_brief()
        }
    }
    pub fn platform_support_brief(&self) -> &'static str {
        let idx = match self.platform {
            Platform::AllwinnerD1Series => "platform-support.allwinner-d1-series",
            Platform::Sophgo2002Series => "platform-support.sophgo-2002-series",
            Platform::NoSpecificPlatform => "platform-support.no-specific-platform",
        };
        locale::get_string(idx, &self.locale)
    }
}

impl From<crate::Config> for App {
    fn from(value: crate::Config) -> Self {
        App {
            locale: value
                .locale
                .unwrap_or_else(|| sys_locale::get_locale().unwrap_or("zh-CN".to_string())),
            bootstrap: value.bootstrap,
            standard_sbi_enabled: value.standard_sbi_enabled.unwrap_or_default(),
            platform: value.platform,
            machine_mode_fdt_ident_enabled: value.machine_fdt_ident_enabled.unwrap_or(true),
            ..Default::default()
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            current_navigation: Route::from_route_id(RouteId::Home),
            navigation_stack: Vec::new(),
            item_length: 0,
            control_flow_fn: None,
            locale: "zh-CN".to_string(),
            bootstrap: Bootstrap::JumpToDram,
            platform: Platform::NoSpecificPlatform,
            standard_sbi_enabled: StandardSbiEnabled::default(),
            supervisor_mode_brief: "",
            bootload_media_brief: "",
            compile_flags_brief: "",
            help_ver_about_brief: "",
            machine_mode_fdt_ident_enabled: true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Language,
    Bootstrap,
    MachineMode,
    SupervisorMode,
    PlatformSupport,
    BootloadMedia,
    CompileFlags,
    HelpVerAbout,
    SampleProgram,
    // machine mode features
    StandardSbiFeat,
    FdtIdent,
    DynamicInfoIdent,
    // route for each platform:
    AllwinnerD1Series,
    Sophgo2002Series,
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub table_state: TableState,
}

impl Route {
    pub fn from_route_id(id: RouteId) -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(usize::MAX));
        Self { id, table_state }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Bootstrap {
    NoBootstrap,
    JumpToDram,

    // Sample programs
    HelloWorld,
    SpiFlash,
}

impl Bootstrap {
    fn is_machine_mode_supported(&self) -> bool {
        match self {
            Bootstrap::JumpToDram => true,
            Bootstrap::NoBootstrap => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardSbiEnabled {
    pub timer: bool,
    pub ipi: bool,
    pub rfence: bool,
    pub hsm: bool,
    pub srst: bool,
    pub pmu: bool,
}

impl StandardSbiEnabled {
    fn sbi_v1p0_ready(&self) -> bool {
        self.timer && self.ipi && self.rfence && self.hsm && self.srst && self.pmu
    }
    fn no_sbi_support(&self) -> bool {
        !self.timer && !self.ipi && !self.rfence && !self.hsm && !self.srst && !self.pmu
    }
}

impl Default for StandardSbiEnabled {
    fn default() -> Self {
        // enable all standard SBI extensions by default.
        StandardSbiEnabled {
            timer: true,
            ipi: true,
            rfence: true,
            hsm: true,
            srst: true,
            pmu: true,
        }
    }
}

pub trait IsSupported {
    fn is_bootstrap_supported(&self, bootstrap: &Bootstrap) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    NoSpecificPlatform,
    AllwinnerD1Series,
    Sophgo2002Series,
}

impl IsSupported for Platform {
    fn is_bootstrap_supported(&self, bootstrap: &Bootstrap) -> bool {
        match (self, bootstrap) {
            (Self::NoSpecificPlatform, Bootstrap::NoBootstrap) => true,
            (Self::NoSpecificPlatform, _) => false,
            (Self::AllwinnerD1Series, Bootstrap::JumpToDram) => true,
            (Self::AllwinnerD1Series, Bootstrap::HelloWorld) => true,
            (Self::AllwinnerD1Series, Bootstrap::SpiFlash) => true,
            (Self::AllwinnerD1Series, Bootstrap::NoBootstrap) => true,
            (Self::Sophgo2002Series, _) => false, // TODO sg2002 support
        }
    }
}

impl IsSupported for Option<Platform> {
    fn is_bootstrap_supported(&self, bootstrap: &Bootstrap) -> bool {
        match self {
            Some(p) => p.is_bootstrap_supported(bootstrap),
            None => false,
        }
    }
}
