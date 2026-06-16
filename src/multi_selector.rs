use std::{
    collections::HashSet,
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveUp, Show},
    event::{Event, KeyCode},
    style::{
        Color::{Black, Reset, Yellow},
        Print, SetBackgroundColor, SetForegroundColor,
    },
};

use crate::common::Error;

pub struct MultiSelector;

impl MultiSelector {
    pub fn run(
        title: String,
        options: Vec<String>,
        default_selections: HashSet<usize>,
    ) -> Result<Option<HashSet<usize>>, Error> {
        crossterm::terminal::enable_raw_mode()?;
        io::stdout().execute(Hide)?;

        let mut selections = default_selections;
        let mut selected_index = 0;
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
                let checkmark = if selections.contains(&i) { "x" } else { " " };

                if i == selected_index {
                    io::stdout()
                        .execute(SetBackgroundColor(Yellow))?
                        .execute(SetForegroundColor(Black))?
                        .execute(Print(format!("[{checkmark}] {option}")))?
                        .execute(SetBackgroundColor(Reset))?
                        .execute(SetForegroundColor(Reset))?;
                } else {
                    print!("[{checkmark}] {option}");
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
                            KeyCode::Char(' ') => {
                                if selections.contains(&selected_index) {
                                    selections.remove(&selected_index);
                                } else {
                                    selections.insert(selected_index);
                                }
                            }
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

        io::stdout().execute(Show)?;
        crossterm::terminal::disable_raw_mode()?;
        io::stdout().flush()?;

        if selected_index < usize::MAX {
            print!(
                "{title} {}\r\n",
                selections
                    .iter()
                    .map(|i| options[*i].as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            Ok(Some(selections))
        } else {
            print!("{title} aborted\r\n");
            Ok(None)
        }
    }
}
