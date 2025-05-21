use std::cmp::Ordering;

pub(crate) struct TreeNode<T> {
    pub(crate) elem: T,
    pub(crate) left: Option<Box<TreeNode<T>>>,
    pub(crate) right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub(crate) fn new(
        elem: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    ) -> TreeNode<T> {
        TreeNode { elem, left, right }
    }
}

impl<T: Ord> TreeNode<T> {
    pub(crate) fn search_insert(&mut self, elem: T) {
        match self.elem.cmp(&elem) {
            Ordering::Less => match &mut self.left {
                Some(left_tree) => left_tree.search_insert(elem),
                None => self.left = Some(Box::new(TreeNode::new(elem, None, None))),
            },
            _ => match &mut self.right {
                Some(right_tree) => right_tree.search_insert(elem),
                None => self.right = Some(Box::new(TreeNode::new(elem, None, None))),
            },
        }
    }

    pub(crate) fn search_contains(&self, elem: T) -> bool {
        match self.elem.cmp(&elem) {
            Ordering::Equal => true,
            Ordering::Less => self
                .left
                .as_ref()
                .map_or(false, |left_tree| left_tree.search_contains(elem)),
            Ordering::Greater => self
                .right
                .as_ref()
                .map_or(false, |right_tree| right_tree.search_contains(elem)),
        }
    }

    pub(crate) fn search_min(&self) -> Option<&T> {
        match &self.left {
            Some(left_tree) => left_tree.search_min(),
            None => Some(&self.elem),
        }
    }

    pub(crate) fn search_max(&self) -> Option<&T> {
        match &self.right {
            Some(right_tree) => right_tree.search_max(),
            None => Some(&self.elem),
        }
    }

    fn remove_max(mut self) -> T {
        match self.right {
            Some(right_tree) => match right_tree.right {
                Some(_) => right_tree.remove_max(),
                None => {
                    self.right = None;
                    right_tree.elem
                }
            },
            None => Some(&self.elem),
        }
    }

    pub(crate) fn search_remove(mut self: Box<Self>, elem: T) -> Option<Box<Self>> {
        match self.elem.cmp(&elem) {
            Ordering::Equal => match (self.left, self.right) {
                (None, None) => None,
                (None, Some(right_tree)) => Some(right_tree),
                (Some(left_tree), None) => Some(left_tree),
                (Some(left_tree), Some(right_tree)) => {}
            },
            Ordering::Less => {
                let left = self
                    .left
                    .take()
                    .and_then(|left_tree| left_tree.search_remove(elem));
                Some(Box::new(TreeNode {
                    elem: self.elem,
                    left,
                    right: self.right,
                }))
            }
            Ordering::Greater => {
                let right = self
                    .right
                    .take()
                    .and_then(|right_tree| right_tree.search_remove(elem));
                Some(Box::new(TreeNode {
                    elem: self.elem,
                    left: self.left,
                    right,
                }))
            }
        }
    }
}
