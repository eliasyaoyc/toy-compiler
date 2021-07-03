//! 652. 寻找重复的子树

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Rc<RefCell<TreeNode>>,
    pub right: Rc<RefCell<TreeNode>>,
}

struct Solution {}

impl Solution {
    pub fn find_duplicate_subtrees() -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Vec::new()
    }
}