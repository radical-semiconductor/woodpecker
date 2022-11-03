use std::io::{self, Result};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Block, Borders},
    Terminal,
    Frame,
};
use crossterm::{
    execute,
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn show_cpu() -> io::Result<()> {
    // set up terminal
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    // fetch backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // perform game loop

    loop {
        terminal.draw(|f| {
            draw_status(f);
        })?;

        let event = read()?;

        if event == Event::Key(KeyCode::Char('q').into()) {
            break
        }

    }

    // restore terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;
    disable_raw_mode()?;

    Ok(())
}


fn draw_status<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, size);
}