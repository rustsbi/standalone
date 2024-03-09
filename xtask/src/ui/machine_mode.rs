use crate::{ui::Builder, App, RouteId};
use ratatui::{layout::Constraint::*, Frame};
use std::ops::ControlFlow;

pub fn draw_machine_mode(f: &mut Frame, app: &mut App) {
    #[rustfmt::skip]
    let items = vec![
        vec!["StandardSbiFeat", "machine-mode.standard-sbi-feat", app.standard_sbi_brief(), ">"],
        vec!["FdtIdent", "machine-mode.fdt-ident", app.fdtident_brief(), ">"],
        vec!["DynamicInfoIdent", "machine-mode.dynamic-info-ident", app.dynamicinfoident_brief(), ">"],
        vec!["Back", "back", "", ""],
    ];
    fn machine_mode_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.push_route(RouteId::StandardSbiFeat),
            1 => app.push_route(RouteId::FdtIdent),
            2 => app.push_route(RouteId::DynamicInfoIdent),
            3 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "machine-mode.title",
        header: vec!["id", "home.item", "home.brief", ""],
        items,
        item_translate_idx: vec![1],
        widths: vec![Length(18), Length(20), Length(30), Min(2)],
        control_flow_fn: machine_mode_handle,
    }
    .draw(f, app)
}
