use crate::common::Error;
use chrono::{Datelike, Local, NaiveDate};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveUp, Show},
    event::{Event, KeyCode},
    style::{
        Color::{Black, DarkGrey, Reset, White, Yellow},
        Print, SetBackgroundColor, SetForegroundColor,
    },
};
use std::{
    io::{self, Write},
    time::Duration,
};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn days_in_month(month: u32, year: i32) -> u32 {
    if month == 2 {
        if year % 4 == 0 && (year % 400 == 0 || year % 100 != 0) {
            MONTH_DAYS[(month - 1) as usize] + 1
        } else {
            MONTH_DAYS[(month - 1) as usize]
        }
    } else {
        MONTH_DAYS[(month - 1) as usize]
    }
}

fn adjust_day(day: u32, month: u32, year: i32, adjustment: i16) -> u32 {
    ((day as i16 + days_in_month(month, year) as i16 + adjustment)
        % days_in_month(month, year) as i16) as u32
}

pub struct DateSelector;

impl DateSelector {
    pub fn run(title: String, default_date: Option<NaiveDate>) -> Result<Option<NaiveDate>, Error> {
        crossterm::terminal::enable_raw_mode()?;
        io::stdout().execute(Hide)?;

        let date = default_date.unwrap_or(Local::now().date_naive());
        let mut year = date.year();
        let mut month = date.month();
        let mut day = date.day();
        let mut date_part_index = 0;
        let mut abort = false;

        print!("{title}\r\n");

        'outer: loop {
            for i in -1..=1 {
                for j in 0..3 {
                    let text = match j {
                        0 => format!("{:04}", year + i),
                        1 => format!("{:>9}", MONTHS[((month as i32 + i + 11) % 12) as usize]),
                        2 => format!(
                            "{:02}",
                            ((day as i32 + i - 1 + days_in_month(month, year) as i32)
                                % days_in_month(month, year) as i32)
                                + 1
                        ),
                        _ => panic!(),
                    };

                    let separator = if j == 2 { "" } else { " : " };
                    let background_color = if j == date_part_index { Yellow } else { White };

                    if i == 0 {
                        io::stdout()
                            .execute(SetBackgroundColor(background_color))?
                            .execute(SetForegroundColor(Black))?
                            .execute(Print(format!("{text}")))?
                            .execute(SetBackgroundColor(Reset))?
                            .execute(SetForegroundColor(Reset))?;
                    } else {
                        io::stdout()
                            .execute(SetForegroundColor(DarkGrey))?
                            .execute(Print(format!("{text}")))?
                            .execute(SetForegroundColor(Reset))?;
                    }

                    if i == 0 {
                        io::stdout()
                            .execute(SetBackgroundColor(White))?
                            .execute(SetForegroundColor(Black))?
                            .execute(Print(format!("{separator}")))?
                            .execute(SetBackgroundColor(Reset))?
                            .execute(SetForegroundColor(Reset))?;
                    } else {
                        io::stdout()
                            .execute(SetForegroundColor(DarkGrey))?
                            .execute(Print(format!("{separator}")))?
                            .execute(SetForegroundColor(Reset))?;
                    }
                }
                print!("\r\n");
            }
            io::stdout().flush()?;

            loop {
                if crossterm::event::poll(Duration::from_millis(100))? {
                    let event = crossterm::event::read()?;
                    match event {
                        Event::Key(key_event) => match key_event.code {
                            KeyCode::Enter => break 'outer,
                            KeyCode::Esc => {
                                abort = true;
                                break 'outer;
                            }
                            KeyCode::Up => match date_part_index {
                                0 => {
                                    year -= 1;
                                    day = day.min(days_in_month(month, year));
                                }
                                1 => {
                                    month = ((month + 12 - 2) % 12) + 1;
                                    day = day.min(days_in_month(month, year));
                                }
                                2 => day = adjust_day(day, month, year, -1),
                                _ => {}
                            },
                            KeyCode::Down => match date_part_index {
                                0 => {
                                    year += 1;
                                    day = day.min(days_in_month(month, year));
                                }
                                1 => {
                                    month = (month % 12) + 1;
                                    day = day.min(days_in_month(month, year));
                                }
                                2 => day = adjust_day(day, month, year, 1),
                                _ => {}
                            },
                            KeyCode::Left => date_part_index = (date_part_index + 3 - 1) % 3,
                            KeyCode::Right => date_part_index = (date_part_index + 1) % 3,
                            _ => {}
                        },
                        _ => {}
                    }

                    break;
                }
            }

            for _ in 0..3 {
                io::stdout().execute(MoveUp(1))?;
                io::stdout().execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::CurrentLine,
                ))?;
            }
        }

        for _ in 0..3 {
            io::stdout().execute(MoveUp(1))?;
            io::stdout().execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine,
            ))?;
        }

        io::stdout().execute(Show)?;
        crossterm::terminal::disable_raw_mode()?;
        io::stdout().flush()?;

        if !abort {
            print!("{title} {year}:{month}:{day}\r\n");
            Ok(Some(NaiveDate::from_ymd_opt(year, month, day).unwrap()))
        } else {
            print!("{title} aborted\r\n");
            Ok(None)
        }
    }
}
