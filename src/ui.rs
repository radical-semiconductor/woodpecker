use bitvec::vec::BitVec;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    cpu::{Command, Cpu},
    error::CpuError,
};

pub fn draw_title<B: Backend>(f: &mut Frame<B>, size: Rect) {
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

const BYTES_PER_LINE: usize = 4;

pub fn draw_status<B: Backend>(
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

    draw_step(f, chunks[0], cpu.step, final_step);
    draw_cmd(f, chunks[1], cpu.get_command());
    draw_err(f, chunks[2], cpu.step, final_step, run_result);
}

fn draw_step<B: Backend>(f: &mut Frame<B>, size: Rect, step: usize, final_step: usize) {
    let step_text = Spans::from(vec![
        Span::styled(
            format!(" {}", step),
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

    f.render_widget(step_widget, size);
}

fn draw_cmd<B: Backend>(f: &mut Frame<B>, size: Rect, cmd: Option<Command>) {
    let cmd_str = match cmd {
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

    f.render_widget(cmd_widget, size);
}

fn draw_err<B: Backend>(
    f: &mut Frame<B>,
    size: Rect,
    step: usize,
    final_step: usize,
    run_result: &std::result::Result<(), CpuError>,
) {
    let (is_error, error_str) = if step == final_step {
        match run_result {
            Ok(_) => (false, String::from("[none]")),
            Err(CpuError::OutOfMemory) => (true, String::from("MEM")),
            Err(CpuError::NegativeAddr) => (true, String::from("NEG")),
        }
    } else {
        (false, String::from("[none]"))
    };

    let mut err_style = Style::default().fg(if is_error {
        Color::LightRed
    } else {
        Color::Gray
    });

    if is_error {
        err_style = err_style.add_modifier(Modifier::BOLD);
    }

    let err_text = Span::styled(error_str, err_style);
    let err_block = Block::default().title("Error").borders(Borders::ALL);
    let err_widget = Paragraph::new(err_text)
        .block(err_block)
        .alignment(Alignment::Center);

    f.render_widget(err_widget, size);
}

pub fn draw_registers<B: Backend>(f: &mut Frame<B>, size: Rect, cpu: &Cpu) {
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

    let store_text = Span::styled(
        cpu.store.to_string(),
        Style::default().fg(Color::LightYellow),
    );
    let store_block = Block::default().title("Store").borders(Borders::ALL);
    let store_widget = Paragraph::new(store_text)
        .block(store_block)
        .alignment(Alignment::Center);

    f.render_widget(addr_widget, chunks[0]);
    f.render_widget(store_widget, chunks[1]);
}

pub fn draw_memory<B: Backend>(f: &mut Frame<B>, size: Rect, cpu: &Cpu) {
    let memory_strs = get_memory_strs(&cpu.memory);
    let memory_text = memory_strs
        .iter()
        .map(|mem_line| {
            Spans::from(Span::styled(
                mem_line,
                Style::default().fg(Color::LightYellow),
            ))
        })
        .collect::<Vec<Spans>>();
    let memory_block = Block::default().title("Memory").borders(Borders::ALL);
    let memory_widget = Paragraph::new(memory_text)
        .block(memory_block)
        .alignment(Alignment::Center);

    f.render_widget(memory_widget, size);
}

fn get_memory_strs(mem: &BitVec) -> Vec<String> {
    let mut byte_strs: Vec<String> = mem
        .chunks(8)
        .map(|c| c.iter().map(|b| if *b { '1' } else { '0' }).collect())
        .collect();

    // complete the last byte with underscores
    let last_byte = byte_strs.last_mut().unwrap();
    let num_missing = 8 - last_byte.len();
    last_byte.extend((0..num_missing).map(|_| '_'));

    // get the chunks of bytes
    let byte_chunks = byte_strs.chunks(BYTES_PER_LINE);
    let num_chunks = byte_chunks.len();

    // print each byte chunk as a line
    byte_chunks
        .enumerate()
        .map(|(chunk_num, chunk)| {
            let addr = chunk_num * BYTES_PER_LINE * 8;
            let mut chunk_elts: Vec<String>;

            let full_chunk = if chunk_num == num_chunks - 1 {
                let num_missing = BYTES_PER_LINE - chunk.len();
                chunk_elts = Vec::from(chunk);
                chunk_elts.extend((0..num_missing).map(|_| "_".repeat(8)));
                &chunk_elts
            } else {
                chunk
            };

            format!("[ADDR 0x{:012x}] ", addr) + &full_chunk.join(" ")
        })
        .collect::<Vec<String>>()
}
