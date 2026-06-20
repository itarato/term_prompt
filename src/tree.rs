use crate::common::Error;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveUp, Show},
    event::{Event, KeyCode},
};
use std::{io, time::Duration};

enum Selection {
    None,
    Elem,
    Child {
        index: usize,
        selection: Box<Selection>,
    },
}

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
        let mut lines_printed = 1;

        print!(
            "{}{:indent$}{}\r\n",
            if selected { "> " } else { "" },
            "",
            self.elem.to_tree_item_repr(),
            indent = indent
        );

        if !self.is_empty() {
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
                                }
                            }
                            KeyCode::Down => {
                                if Self::has_more_sibling(&self.root, &selection[1..]) {
                                    *selection.last_mut().unwrap() += 1;
                                }
                            }
                            KeyCode::Left => {
                                if selection.len() > 1 {
                                    selection.pop().unwrap();
                                }
                            }
                            KeyCode::Right => {
                                if Self::has_more_child(&self.root, &selection[1..]) {
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

    fn has_more_child(node: &TreeNode, selection: &[usize]) -> bool {
        if selection.is_empty() {
            !node.children.is_empty()
        } else {
            if selection[0] < node.children.len() {
                Self::has_more_child(&node.children[selection[0]], &selection[1..])
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
