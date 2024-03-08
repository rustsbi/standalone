use crate::{
    app::{App, RouteId},
    ui::Builder,
};
use std::ops::ControlFlow;
use tui::{backend::Backend, layout::Constraint::*, Frame};

pub fn draw_home<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    #[rustfmt::skip]
    let items = vec![
        vec!["Language", "home.language", app.language_brief(), ">"],
        vec!["Bootstrap", "home.bootstrap", app.bootstrap_brief(), ">"],
        vec!["MachineMode", "home.machine-mode", app.machine_mode_brief(), ">"],
        vec!["SupervisorMode", "home.supervisor-mode", app.supervisor_mode_brief, ">"],
        vec!["PlatformSupport", "home.platform-support", app.platform_support_brief(), ">"],
        vec!["BootloadMedia", "home.bootload-media", app.bootload_media_brief, ">"],
        vec!["CompileFlags", "home.compile-flags", app.compile_flags_brief, ">"],
        vec!["HelpVerAbout", "home.help-ver-about", app.help_ver_about_brief, ">"],
        vec!["QuitAndSave", "home.quit-and-save", "", ""],
    ];
    fn home_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.push_route(RouteId::Language),
            1 => app.push_route(RouteId::Bootstrap),
            2 => app.push_route(RouteId::MachineMode),
            3 => app.push_route(RouteId::SupervisorMode),
            4 => app.push_route(RouteId::PlatformSupport),
            5 => app.push_route(RouteId::BootloadMedia),
            6 => app.push_route(RouteId::CompileFlags),
            7 => app.push_route(RouteId::HelpVerAbout),
            8 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "home.title",
        header: vec!["id", "home.item", "home.brief", ""],
        items,
        item_translate_idx: vec![1],
        widths: vec![Min(18), Length(20), Length(30), Min(2)],
        control_flow_fn: home_handle,
    }
    .draw(f, app)
}
