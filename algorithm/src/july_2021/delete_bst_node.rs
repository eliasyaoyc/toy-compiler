//! 450. 删除二叉树中的节点
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::cmp::Ordering;

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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // public TreeNode deleteNode(TreeNode root, int key) {
        //     if (root == null)
        //     return null;
        //
        //     if(root.val < key) {
        //         root.right = deleteNode(root.right,key);
        //         return root;
        //     } else if(root.val > key){
        //         root.left = deleteNode(root.left,key);
        //         return root;
        //     } else {
        //         if (root.left == null) return root.right;
        //         if (root.right == null) return root.left;
        //         TreeNode left = root.left;
        //         TreeNode right = root.right;
        //         while (right.left != null) {
        //             right = right.left;
        //         }
        //         right.left = left;
        //         return root.right;
        //     }
        // }
        None
    }
}