use std::cell::RefCell;
use std::rc::Rc;

mod node;

pub use node::*;

pub struct SceneGraph {
    root: Rc<RefCell<Node>>,
}

impl SceneGraph {
    pub fn new(root: Node) -> Self {
        Self {
            root: Rc::new(RefCell::new(root)),
        }
    }

    pub fn root(&self) -> Rc<RefCell<Node>> {
        self.root.clone()
    }
}
