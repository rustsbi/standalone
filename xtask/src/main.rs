mod app;
mod locale;
mod ui;

use app::{App, RouteId};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, ops::ControlFlow};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let current_route_id = app.current_route().id;
        terminal.draw(|f| match current_route_id {
            RouteId::Home => ui::draw_home(f, &mut app),
            RouteId::Language => ui::draw_language(f, &mut app),
            RouteId::MachineMode => ui::draw_machine_mode(f, &mut app),
            RouteId::PlatformSupport => ui::draw_platform_support(f, &mut app),
            RouteId::StandardSbiFeat => ui::draw_standard_sbi_features(f, &mut app),
            _ => todo!(),
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter | KeyCode::Char(' ') => match app.enter() {
                    ControlFlow::Continue(_) => continue,
                    ControlFlow::Break(_) => match app.pop_route() {
                        Some(_) => continue,
                        None => break Ok(()),
                    },
                },
                _ => {}
            }
        }
    }
}
