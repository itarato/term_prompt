use crate::common::Error;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveUp, Show},
    event::{Event, KeyCode},
    style::{
        Color::{Black, Reset, White, Yellow},
        SetBackgroundColor, SetForegroundColor,
    },
};
use std::{io, time::Duration};

pub trait TreeNodeItem {
    fn children(&self) -> Vec<Box<dyn TreeNodeItem>>;
    fn to_tree_item_repr(&self) -> String;
}

pub struct TreeNode {
    elem: Box<dyn TreeNodeItem>,
    children: Vec<TreeNode>,
    is_open: bool,
}

impl TreeNode {
    pub fn new(elem: Box<dyn TreeNodeItem>) -> Self {
        let children = elem
            .children()
            .into_iter()
            .map(|child| TreeNode::new(child))
            .collect();

        Self {
            elem,
            children,
            is_open: false,
        }
    }

    fn print(&self, indent: usize, selection: &[usize], self_index: usize) -> usize {
        let selected = !selection.is_empty() && selection[0] == self_index;
        let last_selected = selected && selection.len() == 1;
        let mut lines_printed = 1;

        let toggle_sign = if !self.children.is_empty() && !self.is_open {
            "+"
        } else {
            "-"
        };

        print!("{:indent$}{} ", "", toggle_sign, indent = indent);

        if selected {
            let bg_color = if last_selected { Yellow } else { White };
            io::stdout()
                .execute(SetBackgroundColor(bg_color))
                .unwrap()
                .execute(SetForegroundColor(Black))
                .unwrap();
        }

        print!("{}", self.elem.to_tree_item_repr());

        io::stdout()
            .execute(SetBackgroundColor(Reset))
            .unwrap()
            .execute(SetForegroundColor(Reset))
            .unwrap();

        print!("\r\n");

        if !self.is_empty() && self.is_open {
            let selection = if selected { selection } else { &selection[..0] };

            for (i, child) in self.children.iter().enumerate() {
                lines_printed += child.print(indent + 2, &selection[selection.len().min(1)..], i);
            }
        }

        lines_printed
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

pub struct TreeWalker {
    root: TreeNode,
}

impl TreeWalker {
    pub fn new<T>(node: T) -> Self
    where
        T: TreeNodeItem + 'static,
    {
        Self {
            root: TreeNode::new(Box::new(node)),
        }
    }

    pub fn navigate(&mut self) -> Result<(), Error> {
        let mut selection: Vec<usize> = vec![0];

        crossterm::terminal::enable_raw_mode()?;
        io::stdout().execute(Hide)?;

        let mut previous_lines_printed = 0;

        'outer: loop {
            for _ in 0..previous_lines_printed {
                io::stdout().execute(MoveUp(1))?;
                io::stdout().execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::CurrentLine,
                ))?;
            }
            previous_lines_printed = self.root.print(0, &selection, 0);

            loop {
                if crossterm::event::poll(Duration::from_millis(100))? {
                    let event = crossterm::event::read()?;
                    match event {
                        Event::Key(key_event) => match key_event.code {
                            KeyCode::Enter => break 'outer,
                            KeyCode::Esc => {
                                break 'outer;
                            }
                            KeyCode::Up => {
                                if *selection.last().unwrap() > 0 {
                                    *selection.last_mut().unwrap() -= 1;
                                } else if selection.len() > 1 {
                                    // Go to parent.
                                    selection.pop().unwrap();
                                }
                            }
                            KeyCode::Down => {
                                if Self::has_more_sibling(&self.root, &selection[1..]) {
                                    *selection.last_mut().unwrap() += 1;
                                } else {
                                    // Find first parent with a next child.
                                    let mut sub_selection: &[usize] =
                                        &selection[..selection.len() - 1];
                                    while !sub_selection.is_empty() {
                                        if Self::has_more_sibling(&self.root, sub_selection) {
                                            selection = sub_selection.to_vec();
                                            *selection.last_mut().unwrap() += 1;
                                            break;
                                        }

                                        sub_selection = &sub_selection[..sub_selection.len() - 1];
                                    }
                                }
                            }
                            KeyCode::Left => {
                                if selection.len() > 1 {
                                    selection.pop().unwrap();
                                }
                            }
                            KeyCode::Right => {
                                if Self::has_more_child(&mut self.root, &selection[1..]) {
                                    selection.push(0);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }

                    break;
                }
            }
        }

        io::stdout().execute(Show)?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    fn has_more_child(node: &mut TreeNode, selection: &[usize]) -> bool {
        if selection.is_empty() {
            node.is_open = true;
            !node.children.is_empty()
        } else {
            if selection[0] < node.children.len() {
                node.is_open = true;
                Self::has_more_child(&mut node.children[selection[0]], &selection[1..])
            } else {
                false
            }
        }
    }

    fn has_more_sibling(node: &TreeNode, selection: &[usize]) -> bool {
        if selection.is_empty() {
            false
        } else if selection.len() == 1 {
            node.children.len() - 1 > selection[0]
        } else {
            if selection[0] < node.children.len() {
                Self::has_more_sibling(&node.children[selection[0]], &selection[1..])
            } else {
                false
            }
        }
    }
}
