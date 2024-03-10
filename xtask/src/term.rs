//! Terminal UI program
use crate::{
    app::{App, RouteId},
    ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io, ops::ControlFlow};

pub fn terminal_main(app: &mut App) -> Result<(), Box<dyn Error>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

    let res = run_app(&mut terminal, app);

    reset_terminal()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// Ref: ratatui examples/panic.rs
fn reset_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        let current_route_id = app.current_route().id;
        terminal.draw(|f| match current_route_id {
            RouteId::Home => ui::draw_home(f, app),
            RouteId::Language => ui::draw_language(f, app),
            RouteId::Bootstrap => ui::draw_bootstrap(f, app),
            RouteId::SampleProgram => ui::draw_sample_program(f, app),
            RouteId::MachineMode => ui::draw_machine_mode(f, app),
            RouteId::PlatformSupport => ui::draw_platform_support(f, app),
            RouteId::AllwinnerD1Series => ui::draw_allwinner_d1_series(f, app),
            RouteId::Sophgo2002Series => ui::draw_sophgo_2002_series(f, app),
            RouteId::StandardSbiFeat => ui::draw_standard_sbi_features(f, app),
            RouteId::FdtIdent => ui::draw_fdt_ident(f, app),
            _ => todo!(),
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter | KeyCode::Char(' ') => match app.enter() {
                    ControlFlow::Continue(_) => continue,
                    ControlFlow::Break(_) => match app.pop_route() {
                        Some(_) => continue,
                        None => break Ok(()),
                    },
                },
                KeyCode::Char('q') => {
                    // TODO: Add pop alert window in the future
                    break Ok(());
                }
                // fixme: add Char('/') for search
                _ => {}
            }
        }
    }
}

// impl App {
//     pub fn new(document: &Document) -> io::Result<Self> {
//         let locale = match document.get("language") {
//             Some(Item::Value(Value::String(formatted_string))) => formatted_string.to_string(),
//             Some(_) => {
//                 eprintln!("failed to parse mainfest for key 'language'; string expected");
//                 return Err(io""E)
//             }
//             None => sys_locale::get_locale().unwrap_or("zh-CN".to_string())
//         };
//         App {
//             locale,
//             ..Default::default()
//         }
//     }

//     pub fn save(&self, document: &mut Document) {

//     }
// }
