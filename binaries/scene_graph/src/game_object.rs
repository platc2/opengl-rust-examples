use crate::transform::Transform;
use std::rc::Rc;

pub struct GameObject {
    transform: Transform,
    children: Vec<Rc<Self>>,
}

impl GameObject {
    #[must_use]
    pub fn empty(transform: Transform) -> Self {
        Self {
            transform,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Rc<Self>) {
        self.children.push(child.clone());
    }
}
