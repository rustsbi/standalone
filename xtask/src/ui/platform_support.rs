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
        vec!["NoSpecificPlatform".to_string(), "".to_string(), "".to_string(), ">".to_string()],
        vec!["AllwinnerD1Series".to_string(), "platform-support.allwinner-d1-series".to_string(), "".to_string(), ">".to_string()],
        vec!["Sophgo2002Series".to_string(), "platform-support.sophgo-2002-series".to_string(), "".to_string(), ">".to_string()],
        vec!["Back".to_string(), "back".to_string(), "".to_string(), "".to_string()],
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
