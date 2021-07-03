//! 114 二叉树展开为链表

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

struct Solution {}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(ref mut node) = root{
            Solution::flatten(&mut node.borrow_mut().left);
            Solution::flatten(&mut node.borrow_mut().right);

            let right = node.borrow_mut().right.take();
            let left = node.borrow_mut().left.take();

            node.borrow_mut().right = left;

            let mut p = node.clone();
            while p.borrow().right.is_some(){
                let cur = p.borrow().right.clone().unwrap();
                p = cur;
            }
            p.borrow_mut().right = right;
        }
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn t(){
        println!("1");
    }
}