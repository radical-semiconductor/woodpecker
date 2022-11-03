use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::Result;

use crate::{cpu::Cpu, error::CpuError};

pub fn interact_cpu(cpu: &mut Cpu, run_result: std::result::Result<(), CpuError>) -> Result<()> {
    // mark the final step of the CPU to not go past
    let final_step = cpu.step;

    // set up terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    // fetch backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // perform game loop
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            draw_title(f, chunks[0]);
            draw_status(f, chunks[1], cpu, final_step, &run_result);
            draw_registers(f, chunks[2], cpu);

            let memory_block = Block::default().title("Memory").borders(Borders::ALL);
            f.render_widget(memory_block, chunks[3]);
        })?;

        let event = read()?;

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Left => cpu.backward().unwrap(),
                KeyCode::Right => cpu.forward().unwrap(),
                _ => (),
            }
        }
    }

    // restore terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    disable_raw_mode()?;

    Ok(())
}

fn draw_title<B: Backend>(f: &mut Frame<B>, size: Rect) {
    let status_text = vec![
        Spans::from(vec![
            Span::styled("Arbor Microtech", Style::default().fg(Color::White)),
            Span::raw(" "),
            Span::styled(
                "WoodSIM(R)",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(Span::styled(
            "[Licensed to Kaizen Security]",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let status_widget = Paragraph::new(status_text).alignment(Alignment::Center);

    f.render_widget(status_widget, size);
}

fn draw_status<B: Backend>(
    f: &mut Frame<B>,
    size: Rect,
    cpu: &Cpu,
    final_step: usize,
    run_result: &std::result::Result<(), CpuError>,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(size);

    let step_text = Spans::from(vec![
        Span::styled(
            format!(" {}", cpu.step),
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" / {}", final_step),
            Style::default().fg(Color::LightGreen),
        ),
    ]);
    let step_block = Block::default().title("Step").borders(Borders::ALL);
    let step_widget = Paragraph::new(step_text)
        .block(step_block)
        .alignment(Alignment::Center);

    let cmd_str = match cpu.get_command() {
        Some(cmd) => cmd.to_string(),
        None => String::from("<INIT>"),
    };

    let cmd_text = Span::styled(
        cmd_str,
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    );
    let cmd_block = Block::default().title("Command").borders(Borders::ALL);
    let cmd_widget = Paragraph::new(cmd_text)
        .block(cmd_block)
        .alignment(Alignment::Center);

    let (is_error, error_str) = if cpu.step == final_step {
        match run_result {
            Ok(_) => (false, String::from("[none]")),
            Err(CpuError::OutOfMemory) => (true, String::from("MEM")),
            Err(CpuError::NegativeAddr) => (true, String::from("NEG")),
        }
    } else {
        (false, String::from("[none]"))
    };

    let mut err_style = Style::default()
        .fg(if is_error { Color::LightRed } else { Color::Gray });

    if is_error {
        err_style = err_style.add_modifier(Modifier::BOLD);
    }

    let err_text = Span::styled(error_str, err_style);
    let err_block = Block::default().title("Error").borders(Borders::ALL);
    let err_widget = Paragraph::new(err_text)
        .block(err_block)
        .alignment(Alignment::Center);

    f.render_widget(step_widget, chunks[0]);
    f.render_widget(cmd_widget, chunks[1]);
    f.render_widget(err_widget, chunks[2]);
}

fn draw_registers<B: Backend>(f: &mut Frame<B>, size: Rect, cpu: &Cpu) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(size);

    let addr_text = Span::styled(
        format!("0x{:012x}", cpu.addr),
        Style::default().fg(Color::LightYellow),
    );
    let addr_block = Block::default().title("Address").borders(Borders::ALL);
    let addr_widget = Paragraph::new(addr_text)
        .block(addr_block)
        .alignment(Alignment::Center);

    let store_text = Span::styled(cpu.store.to_string(), Style::default().fg(Color::LightYellow));
    let store_block = Block::default().title("Store").borders(Borders::ALL);
    let store_widget = Paragraph::new(store_text)
        .block(store_block)
        .alignment(Alignment::Center);

    f.render_widget(addr_widget, chunks[0]);
    f.render_widget(store_widget, chunks[1]);
}

/* fn draw_data<B: Backend>(f: &mut Frame<B>, size: u16) {
    let size = f.size();
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, size);
} */
