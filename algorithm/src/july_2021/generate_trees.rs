//! 95. 不同的二叉搜索树
// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
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
      right: None
    }
  }
}

struct Solution{}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Solution::gen_trees(1, n)

    }
    fn gen_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None,];
        }
        let mut allTrees = Vec::new();
        for i in start..=end {
            let leftTrees = Solution::gen_trees(start, i - 1);
            let rightTrees = Solution::gen_trees(i + 1, end);
            for l in &leftTrees {
                for r in &rightTrees {
                    let currentTree = Rc::new(RefCell::new(TreeNode::new(i)));
                    currentTree.borrow_mut().left = l.clone();
                    currentTree.borrow_mut().right = r.clone();
                    allTrees.push(Some(currentTree));
                }
            }
        }
        allTrees
    }
}