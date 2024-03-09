//! Terminal UI program
use crate::{
    app::{App, RouteId},
    ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io, ops::ControlFlow};

pub fn terminal_main(app: &mut App) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = TerminalWrapper {
        inner: Terminal::new(backend)?,
    };

    let res = run_app(terminal.as_mut(), app);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn free_terminal<B: Backend + io::Write>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
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
                    break Ok(())
                },
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

// allow to execute LeaveAlternateScreen and DisableMouseCapture when terminal panics.
struct TerminalWrapper<B: Backend + io::Write> {
    inner: Terminal<B>,
}

impl<B: Backend + io::Write> Drop for TerminalWrapper<B> {
    fn drop(&mut self) {
        if let Err(err) = free_terminal(&mut self.inner) {
            eprintln!("Failed to free terminal: {}", err);
        }
    }
}

impl<B: Backend + io::Write> AsRef<Terminal<B>> for TerminalWrapper<B> {
    fn as_ref(&self) -> &Terminal<B> {
        &self.inner
    }
}

impl<B: Backend + io::Write> AsMut<Terminal<B>> for TerminalWrapper<B> {
    fn as_mut(&mut self) -> &mut Terminal<B> {
        &mut self.inner
    }
}
