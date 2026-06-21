use std::ops::AddAssign;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub(crate) struct SelectionTracker {
    pub total: usize,
    pub selection_line: usize,
    found_selection: bool,
}

impl SelectionTracker {
    pub(crate) fn new() -> Self {
        Self {
            total: 0,
            selection_line: 0,
            found_selection: false,
        }
    }

    pub(crate) fn inc(&mut self, n: usize) {
        self.total += n;

        if !self.found_selection {
            self.selection_line += n;
        }
    }

    pub(crate) fn mark_selection(&mut self) {
        self.found_selection = true;
    }
}

impl AddAssign for SelectionTracker {
    fn add_assign(&mut self, rhs: Self) {
        self.found_selection |= rhs.found_selection;
        self.inc(rhs.total);
    }
}

pub(crate) fn screen_aware_print(lines: Vec<String>, focus_line: usize) {
    let (_term_width, term_height) = crossterm::terminal::size().unwrap();

    if term_height as usize > lines.len() {
        print!("{}\r\n", lines.join("\r\n"));
    } else {
        if term_height as usize / 2 > focus_line {
            print!(
                "{}\r\n\tvvv\tvvv\r\n",
                lines[..term_height as usize].join("\r\n")
            );
        } else if term_height as usize / 2 > lines.len() - focus_line {
            print!(
                "\t^^^\t^^^\r\n{}\r\n",
                lines[lines.len() - term_height as usize..].join("\r\n")
            );
        } else {
            print!(
                "\t^^^\t^^^\r\n{}\r\n\tvvv\tvvv\r\n",
                lines[focus_line - term_height as usize / 2..focus_line + term_height as usize / 2]
                    .join("\r\n")
            );
        }
    }
}
