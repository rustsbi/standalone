use crate::{locale, ui::Builder, App};
use std::ops::ControlFlow;
use tui::{backend::Backend, layout::Constraint::*, Frame};

pub fn draw_language<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let items = vec![
        vec![
            "zh-CN",
            "language.display.zh-CN",
            locale::get_string("language.display.zh-CN", "zh-CN"),
        ],
        vec![
            "en-US",
            "language.display.en-US",
            locale::get_string("language.display.en-US", "en-US"),
        ],
        vec!["Back", locale::get_string("back", &app.locale)],
    ];
    fn language_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        let locale = match idx {
            0 => "zh-CN",
            1 => "en-US",
            2 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        app.locale = locale.to_string();
        ControlFlow::Break(())
    }
    Builder {
        title: "language.title",
        header: vec!["id", "language.language"],
        items,
        item_translate_idx: vec![1],
        widths: vec![Min(18), Length(30), Length(30)],
        control_flow_fn: language_handle,
    }
    .draw(f, app)
}
