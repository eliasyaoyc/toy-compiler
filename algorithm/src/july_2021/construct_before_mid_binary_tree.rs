//! 105.  从前序与中序遍历序列构造二叉树
use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let l = inorder.len();
        let val = preorder[0];
        let index = inorder.iter().position(|&x| x == val).unwrap();
        let mut root = TreeNode::new(val);

        if index > 0 {
            root.left = Solution::build_tree(preorder[1..index + 1].to_vec(), inorder[0..index].to_vec());
        }

        if index < l - 1 {
            root.right = Solution::build_tree(preorder[index + 1..].to_vec(), inorder[index + 1..].to_vec());
        }

        Some(Rc::new(RefCell::new(root)))
    }
}