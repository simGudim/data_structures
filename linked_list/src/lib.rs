use std::boxed::Box;
use std::iter::Iterator;

type NodePtr<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    value: T,
    next: NodePtr<T>
}

pub struct LinkedList<T> {
    head: NodePtr<T>
}

impl <T>From<Node<T>> for NodePtr<T> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

impl <T>LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_head: NodePtr<T> = Node {
            value,
            next: old_head
        }.into();
        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            None => {
                return None
            },
            Some(head) => {
                self.head = head.next;
                return Some(head.value)
            }
        }
    }

    pub fn peak(&self) -> Option<&T> {
        if let Some(ref head) = self.head {
            return Some(&head.value)
        } else{
            return None
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

pub struct IntoIter<T>(LinkedList<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(99);
        list.push(100);


        for i in list.iter_mut() {
            println!("mutable iterator: {:?}", i);
        }
    }
}
