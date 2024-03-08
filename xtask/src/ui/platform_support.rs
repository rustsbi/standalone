use crate::{
    app::{Platform, RouteId},
    ui::Builder,
    App,
};
use std::ops::ControlFlow;
use tui::{backend::Backend, layout::Constraint::*, Frame};

pub fn draw_platform_support<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
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
        widths: vec![Min(18), Length(25), Length(30), Min(2)],
        control_flow_fn: platform_support_handle,
    }
    .draw(f, app)
}
