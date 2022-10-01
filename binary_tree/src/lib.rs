use std::boxed::Box;
use std::collections::VecDeque;

type NodePntr = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    value: i32,
    left: NodePntr,
    right: NodePntr
}

impl From<Node> for NodePntr {
    fn from(node: Node) -> Option<Box<Node>> {
        Some(Box::new(node))
    }
}

#[derive(Debug)]
pub struct BinaryTree {
    root: NodePntr
}

impl BinaryTree {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Node {
                    value,
                    left: None,
                    right: None
                }.into();
            },
            Some(node) => {
                BinaryTree::insert_recursive(node, value)
            }
        }
    }

    pub fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node {
                        value,
                        left: None,
                        right: None
                    }.into();
                },
                Some(left_node) => {
                    BinaryTree::insert_recursive(left_node, value)
                }
            }
        }
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node {
                        value,
                        left: None,
                        right: None
                    }.into();
                },
                Some(right_node) => {
                    BinaryTree::insert_recursive(right_node, value)
                }
            }
        }
    }

    pub fn depth_traversal(&self) -> Vec<i32> {
        let mut result = Vec::new();
        match &self.root {
            None => result,
            Some(node) => BinaryTree::depth_first_traversal(&mut result, node)
        }
    }

    pub fn depth_first_traversal(result: &mut Vec<i32>, node: &Box<Node>) -> Vec<i32> {
        if let Some(ref left_node) = node.left {
            BinaryTree::depth_first_traversal(result, &left_node);
        }
        result.push(node.value);
        if let Some(ref right_node) = node.right {
            BinaryTree::depth_first_traversal(result, &right_node);
        }
        result.to_vec()
    }

    pub fn breadth_first_traversal(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if self.root.is_none() {
            return result
        }

        let mut queue: VecDeque<&Box<Node>> = VecDeque::new();
        let root = self.root.as_ref().unwrap();
        result.push(root.value);
        queue.push_front(root);

        while let Some(node) = queue.pop_front() {
            if let Some(ref left_node) = node.left {
                result.push(left_node.value);
                queue.push_front(left_node);
            }
            if let Some(ref right_node) = node.right {
                result.push(right_node.value);
                queue.push_front(right_node);
            }
        }
        result
    }
}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = BinaryTree::new();
        tree.insert(24);
        tree.insert(32);
        tree.insert(100);
        tree.insert(102);
        tree.insert(12);

        println!("tree: {:?}", tree.breadth_first_traversal());
    }
}
