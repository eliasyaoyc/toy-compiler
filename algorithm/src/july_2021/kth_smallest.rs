//! 230. 二叉搜索树中第 K 小的元素
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
            if let Some(r) = root.as_ref() {
                inorder(r.borrow_mut().left.take(), vec);
                vec.push(r.borrow().val);
                inorder(r.borrow_mut().right.take(), vec);
            }
        }
        let mut res = Vec::<i32>::new();
        inorder(root, &mut res);
        res[k as usize - 1]
    }
}
