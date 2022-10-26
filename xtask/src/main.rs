use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, ops::ControlFlow};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut state = TableState::default();
        state.select(Some(usize::MAX)); // workaround for left margins
        App {
            state,
            #[rustfmt::skip]
            items: vec![
                vec!["Language", "配置语言", "安装配置程序显示的自然语言"],
                vec!["CompileMode", "编译配置", "编译目标、处理器核优化等基本配置"],
                vec!["BundledDriver", "捆绑驱动", "链接在输出包中的机器态驱动程序"],
                vec!["ExecuteMedia", "运行介质", "编译配置支持的存储介质"],
                vec!["StandardFeatures", "标准扩展", "启用或关闭支持的SBI标准扩展"],
                vec!["CustomFeatures", "额外的扩展", "安全环境功能、软模拟虚拟扩展和固件调试功能"],
                vec!["HelpVerAbout", "帮助关于", "本程序的帮助手册、版本号和有关信息"],
                vec!["Quit", "退出程序", "退出此安装配置程序"],
            ],
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn enter(&self) -> ControlFlow<(), ()> {
        match self.state.selected() {
            Some(7) => ControlFlow::Break(()),
            Some(_) => ControlFlow::Continue(()), // todo
            None => ControlFlow::Continue(()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
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
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => match app.enter() {
                    ControlFlow::Continue(_) => continue,
                    ControlFlow::Break(_) => break Ok(()),
                },
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(0)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::LightBlue);
    let header_cells = ["Id", "名称", "帮助"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(0);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(0)
    });
    let t = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" RustSBI / Standard - 编译选项 "),
        )
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Min(18),
            Constraint::Length(20),
            Constraint::Percentage(50),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
