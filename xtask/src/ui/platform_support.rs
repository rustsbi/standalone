use crate::{
    app::{Platform, RouteId},
    ui::Builder,
    App,
};
use ratatui::{layout::Constraint::*, Frame};
use std::ops::ControlFlow;

pub fn draw_platform_support(f: &mut Frame, app: &mut App) {
    #[rustfmt::skip]
    let items = vec![
        vec!["NoSpecificPlatform", "platform-support.no-specific-platform", "", ">"],
        vec!["AllwinnerD1Series", "platform-support.allwinner-d1-series", "", ">"],
        vec!["Sophgo2002Series", "platform-support.sophgo-2002-series", "", ">"],
        vec!["Back", "back", "", ""],
    ];
    fn platform_support_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.platform = Platform::NoSpecificPlatform,
            1 => app.push_route(RouteId::AllwinnerD1Series),
            2 => app.push_route(RouteId::Sophgo2002Series),
            3 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "platform-support.title",
        header: vec!["id", "home.item", "home.brief", ""],
        items,
        item_translate_idx: vec![1],
        widths: vec![Length(18), Length(25), Length(30), Min(2)],
        control_flow_fn: platform_support_handle,
    }
    .draw(f, app)
}
