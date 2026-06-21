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
