//! 106. 从中序与后序遍历序列构造二叉树
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }

        let l = postorder.len();
        let val = postorder[l - 1];
        let index = inorder.iter().position(|&x| x == val).unwrap();
        let mut root = TreeNode::new(val);

        if index > 0 {
            root.left = Solution::build_tree(inorder[0..index].to_vec(), postorder[0..index].to_vec());
        }

        if index < l - 1 {
            root.right = Solution::build_tree(inorder[index + 1..].to_vec(), postorder[index..l - 1].to_vec());
        }

        Some(Rc::new(RefCell::new(root)))
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn t() {
        let v = vec![1, 2, 3, 4];
        let a = v[1..v.len() - 1].to_vec();
        let b = a.last().unwrap();
        println!("{}",b);
    }
}