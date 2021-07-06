//! 98.验证二叉搜索树
use std::rc::Rc;
use std::cell::{RefCell, Ref};

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn valid_bst(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            if let Some(r) = root.as_ref() {
                return min.map_or(true, |x| r.borrow().val > x)
                    && max.map_or(true, |x| r.borrow().val < x)
                    && valid_bst(r.borrow().left.clone(), min, Some(r.borrow().val))
                    && valid_bst(r.borrow().right.clone(), Some(r.borrow().val), max);
            }
            true
        }
        valid_bst(root, None, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::july_2021::is_valid_bst::{TreeNode, Solution};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn t() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))));
    }
}