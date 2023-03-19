use crate::{ui::Builder, App, RouteId};
use std::ops::ControlFlow;
use tui::{backend::Backend, layout::Constraint::*, Frame};

pub fn draw_machine_mode<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    #[rustfmt::skip]
    let items = vec![
        vec!["StandardSbiFeat".to_string(), "machine-mode.standard-sbi".to_string(), app.standard_sbi_brief(), ">".to_string()],
        vec!["Back".to_string(), "back".to_string(), "".to_string(), "".to_string()],
    ];
    fn machine_mode_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.push_route(RouteId::StandardSbiFeat),
            1 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "machine-mode.title",
        header: vec!["id", "home.item", "home.brief", ""],
        items,
        item_translate_idx: vec![1],
        widths: vec![Min(18), Length(20), Length(30), Min(2)],
        control_flow_fn: machine_mode_handle,
    }
    .draw(f, app)
}
