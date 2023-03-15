use std::error::Error;
use std::io;
use tui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use crossterm::terminal::ClearType::All;
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType};

pub fn start_ui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode();
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture);
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block = Block::default()
        .title("stolker")
        .title_alignment(Alignment::Center);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let requests_block = Block::default()
        .title(vec![
            Span::styled("requests", Style::default()),
            Span::from("requests")
        ])
        .style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(requests_block, chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let details_block = Block::default()
        .title(vec![
            Span::styled("details", Style::default()),
            Span::from("details")
        ])
        .style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(details_block, right_chunks[0]);

    let bytes_block = Block::default()
        .title(vec![
            Span::styled("bytes", Style::default()),
            Span::from("details")
        ])
        .style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(bytes_block, right_chunks[1]);
}
