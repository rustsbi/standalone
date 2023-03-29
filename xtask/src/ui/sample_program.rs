use crate::{app::Bootstrap, ui::Builder, App};
use std::ops::ControlFlow;
use tui::{backend::Backend, layout::Constraint::*, Frame};

pub fn draw_sample_program<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    fn choose_str(enabled: bool) -> &'static str {
        match enabled {
            true => "sample-program.chosen",
            false => "sample-program.not-chosen",
        }
    }
    let hello_world = choose_str(matches!(app.bootstrap, Bootstrap::HelloWorld));
    let spi_flash = choose_str(matches!(app.bootstrap, Bootstrap::SpiFlash));
    #[rustfmt::skip]
    let items = vec![
        vec!["HelloWorld".to_string(), "sample-program.hello-world".to_string(), hello_world.to_string()],
        vec!["SpiFlash".to_string(), "sample-program.spi-flash".to_string(), spi_flash.to_string()],
        vec!["Back".to_string(), "back".to_string(), "".to_string()],
    ];
    fn sample_program_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.bootstrap = Bootstrap::HelloWorld,
            1 => app.bootstrap = Bootstrap::SpiFlash,
            2 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "sample-program.title",
        header: vec!["id", "home.item", "home.brief", ""],
        items,
        item_translate_idx: vec![1, 2],
        widths: vec![Min(18), Length(30), Length(20)],
        control_flow_fn: sample_program_handle,
    }
    .draw(f, app)
}
