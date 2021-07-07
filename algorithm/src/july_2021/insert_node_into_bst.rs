//! 701. 二叉搜索树中的插入操作
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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = &root{{
            let mut root = r.borrow_mut();
            if val < root.val {
                root.left=Self::insert_into_bst(root.left.take(),val)
            } else {
                root.right=Self::insert_into_bst(root.right.take(),val)
            }}
            root
        }else{Some(Rc::new(RefCell::new(TreeNode {left:None,right:None,val: val})))}
    }
}