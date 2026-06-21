use term_prompt::tree::{TreeNodeItem, TreeWalker};

#[derive(Clone)]
struct NodeImpl {
    title: String,
    children: Vec<Self>,
}

impl NodeImpl {
    fn new(title: String, children: Vec<Self>) -> Self {
        Self { title, children }
    }
}

impl TreeNodeItem for NodeImpl {
    fn children(&self) -> Vec<Box<dyn TreeNodeItem>> {
        self.children
            .clone()
            .into_iter()
            .map(|child| {
                let boxed: Box<dyn TreeNodeItem> = Box::new(child);
                boxed
            })
            .collect()
    }

    fn to_tree_item_repr(&self) -> String {
        format!("{}", self.title)
    }
}

fn main() {
    println!("\n");

    let level2: Vec<NodeImpl> = (0..100)
        .map(|i| NodeImpl::new(format!("Level 2 Node: {}", i), vec![]))
        .collect();
    let level1: Vec<NodeImpl> = (0..100)
        .map(|i| NodeImpl::new(format!("Level 1 Node: {}", i), level2.clone()))
        .collect();
    let root = NodeImpl::new("Level 0 Node".to_string(), level1);

    TreeWalker::new(root).navigate().unwrap();
}
