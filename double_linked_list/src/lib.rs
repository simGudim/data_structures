use std::rc::{Rc, Weak};
use std::cell::RefCell;


type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T: Copy> {
    value: T,
    previous: NodePtr<T>,
    next: NodePtr<T>
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            previous: None,
            next: None
        }
    }
}

impl<T:Copy> From<Node<T>> for NodePtr<T>{
    fn from(node: Node<T>) -> NodePtr<T> {
        Some(Rc::new(RefCell::new(node)))
    }
}

pub struct List<T: Copy> {
    head: NodePtr<T>,
    tail: NodePtr<T>
}

impl <T:Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn push_back(&mut self, value: T) {
        let node = Node::new(value).into();
        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = node.into();
            },
            Some(current_tail) =>  {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(tail) => {
                let mut tail = tail.borrow_mut();
                let prev = tail.previous.take();
                match prev {
                    None => {
                        self.head.take();
                    },
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                }
                Some(tail.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = List::new();
    }
}
