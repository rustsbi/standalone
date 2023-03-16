use core::mem;
use std::ops::ControlFlow;
use tui::widgets::TableState;

pub struct App {
    current_navigation: Route,
    navigation_stack: Vec<Route>,
    pub item_length: usize,
    pub control_flow_fn: Option<fn(usize, &mut App) -> ControlFlow<(), ()>>,
    pub locale: String,
    pub standard_sbi_enabled: StandardSbiEnabled,
    pub supervisor_mode_brief: String,
    pub platform_support_brief: String,
    pub bootload_media_brief: String,
    pub compile_flags_brief: String,
    pub help_ver_about_brief: String,
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
    pub fn language_brief(&self) -> String {
        crate::locale::get_string("language.display.current", &self.locale).to_string()
    }
    pub fn standard_sbi_brief(&self) -> String {
        let idx = if self.standard_sbi_enabled.sbi_v1p0_ready() {
            "standard-sbi-features.v1p0-prepared"
        } else if self.standard_sbi_enabled.no_sbi_support() {
            "standard-sbi-features.no-support"
        } else {
            "standard-sbi-features.partial"
        };
        crate::locale::get_string(idx, &self.locale).to_string()
    }
    pub fn machine_mode_brief(&self) -> String {
        // in future when we have custom sbi features here, change this function
        self.standard_sbi_brief()
    }
}

impl Default for App {
    fn default() -> Self {
        let locale = sys_locale::get_locale().unwrap_or("zh-CN".to_string());
        App {
            current_navigation: Route::from_route_id(RouteId::Home),
            navigation_stack: Vec::new(),
            item_length: 0,
            control_flow_fn: None,
            locale,
            standard_sbi_enabled: StandardSbiEnabled::default(),
            supervisor_mode_brief: String::new(),
            platform_support_brief: String::new(),
            bootload_media_brief: String::new(),
            compile_flags_brief: String::new(),
            help_ver_about_brief: String::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Language,
    MachineMode,
    SupervisorMode,
    PlatformSupport,
    BootloadMedia,
    CompileFlags,
    HelpVerAbout,
    StandardSbiFeat,
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

#[derive(Debug, Clone)]
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
