mod common;
pub mod selector;

use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    cursor::MoveUp,
    event::{Event, KeyCode},
    style::{
        Color::{Black, Reset, Yellow},
        Print, SetBackgroundColor, SetForegroundColor,
    },
};

use crate::common::Error;

pub struct Selector;

impl Selector {
    pub fn run(
        title: String,
        options: Vec<String>,
        default_selection: Option<usize>,
    ) -> Result<Option<usize>, Error> {
        crossterm::terminal::enable_raw_mode()?;

        let mut selected_index = default_selection.unwrap_or(0);
        let mut previous_printed_line_count = 0;

        print!("{title}\r\n");

        'outer: loop {
            for _ in 0..previous_printed_line_count {
                io::stdout().execute(MoveUp(1))?;
                io::stdout().execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::CurrentLine,
                ))?;
            }

            previous_printed_line_count = 0;

            for (i, option) in options.iter().enumerate() {
                if i == selected_index {
                    io::stdout()
                        .execute(SetBackgroundColor(Yellow))?
                        .execute(SetForegroundColor(Black))?
                        .execute(Print(format!("{i}: {option}")))?
                        .execute(SetBackgroundColor(Reset))?
                        .execute(SetForegroundColor(Reset))?;
                } else {
                    print!("{i}: {option}");
                }
                print!("\r\n");
                previous_printed_line_count += 1;
            }
            io::stdout().flush()?;

            loop {
                if crossterm::event::poll(Duration::from_millis(100))? {
                    let event = crossterm::event::read()?;
                    match event {
                        Event::Key(key_event) => match key_event.code {
                            KeyCode::Enter => break 'outer,
                            KeyCode::Esc => {
                                selected_index = usize::MAX;
                                break 'outer;
                            }
                            KeyCode::Up => {
                                selected_index =
                                    (selected_index + options.len() - 1) % options.len()
                            }
                            KeyCode::Down => selected_index = (selected_index + 1) % options.len(),
                            _ => {}
                        },
                        _ => {}
                    }

                    break;
                }
            }
        }

        for _ in 0..=previous_printed_line_count {
            io::stdout().execute(MoveUp(1))?;
            io::stdout().execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine,
            ))?;
        }

        crossterm::terminal::disable_raw_mode()?;
        io::stdout().flush()?;

        if selected_index < usize::MAX {
            print!("{title} {}\r\n", options[selected_index]);
            Ok(Some(selected_index))
        } else {
            print!("{title} aborted\r\n");
            Ok(None)
        }
    }
}
