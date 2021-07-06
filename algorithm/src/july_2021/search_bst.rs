//! 700. 二叉搜索树中的搜索
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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.as_ref() {
            if r.borrow().val.eq(&val) {
                return root;
            }
            if r.borrow().val.gt(&val) {
                return Solution::search_bst(r.borrow_mut().left.take(), val);
            }
            if r.borrow().val.lt(&val) {
                return Solution::search_bst(r.borrow_mut().right.take(), val);
            }
        }
        None
    }
}