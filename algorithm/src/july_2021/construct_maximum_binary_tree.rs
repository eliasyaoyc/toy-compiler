//! 654. 最大二叉树

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mut i = 0;

        for j in 0..nums.len() {
            if nums[j] > nums[i] {
                i = j;
            }
        }

        let left = Solution::construct_maximum_binary_tree(nums[..i].to_vec());
        let right = Solution::construct_maximum_binary_tree(nums[i + 1..].to_vec());
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[i],
            left,
            right,
        })))
    }
}