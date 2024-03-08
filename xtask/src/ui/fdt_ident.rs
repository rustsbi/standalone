use crate::{ui::Builder, App};
use ratatui::{layout::Constraint::*, Frame};
use std::ops::ControlFlow;

pub fn draw_fdt_ident(f: &mut Frame, app: &mut App) {
    #[rustfmt::skip]
    let items = vec![
        vec!["FdtIdentEnabled", "fdt-ident.fdt-ident-enabled", ""],
        vec!["Back", "back", ""],
    ];
    fn machine_mode_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.machine_mode_fdt_ident_enabled = !app.machine_mode_fdt_ident_enabled,
            1 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "fdt-ident.title",
        header: vec!["id", "home.item", "home.brief"],
        items,
        item_translate_idx: vec![1, 2],
        widths: vec![Length(18), Min(30), Length(12)],
        control_flow_fn: machine_mode_handle,
    }
    .draw(f, app)
}
