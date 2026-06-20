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
    let nodes = NodeImpl::new(
        "Animal".to_string(),
        vec![
            NodeImpl::new(
                "Cat".to_string(),
                vec![
                    NodeImpl::new("Russian blue".to_string(), vec![]),
                    NodeImpl::new("Siamese".to_string(), vec![]),
                    NodeImpl::new("Sphinx".to_string(), vec![]),
                ],
            ),
            NodeImpl::new(
                "Dog".to_string(),
                vec![
                    NodeImpl::new("Leopard hound".to_string(), vec![]),
                    NodeImpl::new("Bulldog".to_string(), vec![]),
                    NodeImpl::new("German shepherd".to_string(), vec![]),
                    NodeImpl::new("Pitbull".to_string(), vec![]),
                ],
            ),
            NodeImpl::new("Slug".to_string(), vec![]),
        ],
    );

    TreeWalker::new(nodes).navigate().unwrap();
}
