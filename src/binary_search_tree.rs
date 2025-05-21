use std::fmt::Display;

use crate::tree_node::TreeNode;

pub struct BinarySearchTree<T: Ord> {
    tree: Option<Box<TreeNode<T>>>,
    length: u32,
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            tree: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, elem: T) {
        match self.length {
            0 => self.tree = Some(Box::new(TreeNode::new(elem, None, None))),
            _ => self.tree.as_mut().unwrap().search_insert(elem),
        }
        self.length += 1;
    }

    pub fn contains(&self, elem: T) -> bool {
        self.tree
            .as_ref()
            .map_or(false, |tree| tree.search_contains(elem))
    }

    pub fn min(&self) -> Option<&T> {
        self.tree.as_ref().and_then(|tree| tree.search_min())
    }

    pub fn max(&self) -> Option<&T> {
        self.tree.as_ref().and_then(|tree| tree.search_max())
    }

    pub fn remove(&mut self, elem: T) {
        self.tree = todo!();
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
