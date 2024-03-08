use crate::{app::Bootstrap, ui::Builder, App};
use ratatui::{layout::Constraint::*, Frame};
use std::ops::ControlFlow;

pub fn draw_sample_program(f: &mut Frame, app: &mut App) {
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
        vec!["HelloWorld", "sample-program.hello-world", hello_world],
        vec!["SpiFlash", "sample-program.spi-flash", spi_flash],
        vec!["Back", "back", ""],
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
        widths: vec![Length(18), Length(30), Length(20)],
        control_flow_fn: sample_program_handle,
    }
    .draw(f, app)
}
