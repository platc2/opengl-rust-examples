use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
    pub name: String,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn add_child(self_rc: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        child.borrow_mut().parent = Some(Rc::downgrade(self_rc));
        self_rc.borrow_mut().children.push(child);
    }

    pub fn children(&self) -> Vec<Rc<RefCell<Node>>> {
        self.children.clone()
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }
}
