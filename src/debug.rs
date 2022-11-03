use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::{
    cpu::Cpu,
    error::CpuError,
    ui::{draw_memory, draw_registers, draw_status, draw_title},
    Result,
};

pub struct CpuDebugger<'a> {
    cpu: &'a mut Cpu,
    run_result: &'a std::result::Result<(), CpuError>,
    step: usize,
    final_step: usize,
}

impl<'a> CpuDebugger<'a> {
    pub fn new(cpu: &'a mut Cpu, run_result: &'a std::result::Result<(), CpuError>) -> Self {
        // we want to start viewing the error if applicable
        let step = cpu.step + if run_result.is_err() { 1 } else { 0 };

        Self {
            cpu,
            run_result,
            step,
            final_step: step,
        }
    }

    pub fn interact(&mut self) -> Result<()> {
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
                draw_status(
                    f,
                    chunks[1],
                    self.cpu,
                    &self.run_result,
                    self.step,
                    self.final_step,
                );
                draw_registers(f, chunks[2], self.cpu);
                draw_memory(f, chunks[3], self.cpu);
            })?;

            let event = read()?;

            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => self.backward()?,
                    KeyCode::Right => self.forward()?,
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

    fn forward(&mut self) -> Result<()> {
        if self.step < self.final_step {
            self.step += 1;

            // make sure we don't step the CPU into an error
            if !(self.step == self.final_step && self.run_result.is_err()) {
                self.cpu.forward()?;
            }
        }

        Ok(())
    }

    fn backward(&mut self) -> Result<()> {
        if self.step > 0 {
            // make sure we don't try to step the CPU out of an error
            if !(self.step == self.final_step && self.run_result.is_err()) {
                self.cpu.backward()?;
            }

            self.step -= 1;
        }

        Ok(())
    }
}
