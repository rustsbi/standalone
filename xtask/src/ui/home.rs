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
        vec!["Language".to_string(), "home.language".to_string(), app.language_brief(), ">".to_string()],
        vec!["MachineMode".to_string(), "home.machine-mode".to_string(), app.machine_mode_brief(), ">".to_string()],
        vec!["SupervisorMode".to_string(), "home.supervisor-mode".to_string(), app.supervisor_mode_brief.to_string(), ">".to_string()],
        vec!["PlatformSupport".to_string(), "home.platform-support".to_string(), app.platform_support_brief.to_string(), ">".to_string()],
        vec!["BootloadMedia".to_string(), "home.bootload-media".to_string(), app.bootload_media_brief.to_string(), ">".to_string()],
        vec!["CompileFlags".to_string(), "home.compile-flags".to_string(), app.compile_flags_brief.to_string(), ">".to_string()],
        vec!["HelpVerAbout".to_string(), "home.help-ver-about".to_string(), app.help_ver_about_brief.to_string(), ">".to_string()],
        vec!["Quit".to_string(), "home.quit".to_string(), "".to_string(), "".to_string()],
    ];
    fn home_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.push_route(RouteId::Language),
            1 => app.push_route(RouteId::MachineMode),
            2 => app.push_route(RouteId::SupervisorMode),
            3 => app.push_route(RouteId::PlatformSupport),
            4 => app.push_route(RouteId::BootloadMedia),
            5 => app.push_route(RouteId::CompileFlags),
            6 => app.push_route(RouteId::HelpVerAbout),
            7 => return ControlFlow::Break(()),
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
