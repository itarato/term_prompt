use std::ops::AddAssign;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub(crate) struct SelectionTracker {
    pub index: usize,
    found_selection: bool,
}

impl SelectionTracker {
    pub(crate) fn new() -> Self {
        Self {
            index: 1,
            found_selection: false,
        }
    }

    pub(crate) fn mark_selection(&mut self) {
        self.found_selection = true;
    }
}

impl AddAssign for SelectionTracker {
    fn add_assign(&mut self, rhs: Self) {
        if self.found_selection {
            return;
        }

        self.found_selection = rhs.found_selection;
        self.index += rhs.index;
    }
}

pub(crate) fn screen_aware_print(lines: Vec<String>, focus_line: usize) -> usize {
    let (_term_width, term_height) = crossterm::terminal::size().unwrap();
    let height = term_height as usize;
    let half_height = term_height as usize / 2 - 1;

    if height > lines.len() {
        print!("{}\r\n", lines.join("\r\n"));
        lines.len()
    } else {
        if half_height > focus_line {
            let to = height - 2;
            print!("{}\r\n\t...more\r\n", lines[..to].join("\r\n"));
            to + 1
        } else if half_height > lines.len() - focus_line {
            let from = lines.len() - height + 2;
            print!("\t...more\r\n{}\r\n", lines[from..].join("\r\n"));
            lines.len() - from + 1
        } else {
            let from = focus_line - half_height;
            let to = (from + height - 3).min(lines.len());
            print!(
                "\t...more\r\n{}\r\n\t...more\r\n",
                lines[from..to].join("\r\n")
            );
            to - from + 2
        }
    }
}
