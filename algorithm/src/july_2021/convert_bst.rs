//! 538 & 1038. 把二叉搜索树转换为累加树

use std::rc::Rc;
use std::cell::RefCell;

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

pub struct Solution {}

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn in_order_rev(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = node {
                in_order_rev(node.borrow().right.clone(), sum);
                node.borrow_mut().val += *sum;
                *sum = node.borrow().val;
                in_order_rev(node.borrow().left.clone(), sum)
            }
        }
        in_order_rev(root.clone(), &mut 0);
        root
    }
}