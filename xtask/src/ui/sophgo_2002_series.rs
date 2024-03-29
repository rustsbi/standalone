use crate::{app::Platform, ui::Builder, App};
use ratatui::{layout::Constraint::*, Frame};
use std::ops::ControlFlow;

pub fn draw_sophgo_2002_series(f: &mut Frame, app: &mut App) {
    fn choose_str(enabled: bool) -> &'static str {
        match enabled {
            true => "platform-support.chosen",
            false => "platform-support.not-chosen",
        }
    }
    let choose_platform = choose_str(matches!(app.platform, Platform::Sophgo2002Series));
    #[rustfmt::skip]
    let items = vec![
        vec!["ChoosePlatform", "platform-support.choose-platform", choose_platform, ""],
        vec!["Back", "back", "", ""],
    ];
    fn machine_mode_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.platform = Platform::Sophgo2002Series,
            1 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "sophgo-2002-series.title",
        header: vec!["id", "home.item", "home.brief", ""],
        items,
        item_translate_idx: vec![1, 2],
        widths: vec![Length(18), Length(20), Length(30), Min(2)],
        control_flow_fn: machine_mode_handle,
    }
    .draw(f, app)
}
