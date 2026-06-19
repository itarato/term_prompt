use std::io;

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
};

use crate::common::Error;

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

    pub fn navigate(&mut self) -> Result<(), Error> {
        crossterm::terminal::enable_raw_mode()?;
        io::stdout().execute(Hide)?;

        loop {}

        io::stdout().execute(Show)?;

        Ok(())
    }

    fn print(&self, indent: usize) {
        println!(
            "{:indent$}{}",
            "",
            self.elem.to_tree_item_repr(),
            indent = indent
        );

        for child in &self.children {
            let open_sign = if child.is_empty() {
                "-"
            } else if child.is_open {
                "="
            } else {
                "+"
            };
            println!(
                "{:indent$}  {open_sign} {}",
                "",
                child.elem.to_tree_item_repr(),
                indent = indent
            );
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}
