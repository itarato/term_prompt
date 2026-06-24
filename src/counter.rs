use std::io::{self, Write};

use crossterm::ExecutableCommand;

pub struct Counter {
    title: String,
    counter: i64,
    is_completed: bool,
}

impl Counter {
    pub fn new(title: String) -> Self {
        println!("\r\n");

        Self {
            title,
            counter: 0,
            is_completed: false,
        }
    }

    pub fn inc(&mut self, delta: i64) {
        self.counter += delta;
        self.refresh_screen();
    }

    pub fn complete(&mut self) {
        self.is_completed = true;
        self.refresh_screen();
    }

    fn refresh_screen(&self) {
        io::stdout()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine,
            ))
            .unwrap();

        print!("\r{}: {}", self.title, self.counter);

        if self.is_completed {
            print!(" [done]\r\n");
        }

        io::stdout().flush().unwrap();
    }
}
