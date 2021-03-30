use std::{cell::RefCell, rc::Rc};

use crate::types::Type;

pub struct Scope<'a> {
    head: Option<Rc<Node<'a>>>,
}

struct Node<'a> {
    prev: Option<Rc<Node<'a>>>,
    name: &'a str,
    ty: Rc<RefCell<Type>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope { head: None }
    }
    pub fn insert(&mut self, name: &'a str, ty: Rc<RefCell<Type>>) {
        self.head = Some(Rc::new(Node {
            name,
            ty,
            prev: self.head
        }))
    }
    pub fn get(&self, name: &'a str) -> Option<&Rc<RefCell<Type>>> {
        let node = self.head?;
        loop {
            if node.name == name {
                return Some(&node.ty)
            }
            node = node.prev?;
        }
    }
}