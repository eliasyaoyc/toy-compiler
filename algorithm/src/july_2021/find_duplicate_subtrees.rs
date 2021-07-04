//! 652. 寻找重复的子树

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Rc<RefCell<TreeNode>>,
    pub right: Rc<RefCell<TreeNode>>,
}

struct Solution {}

impl Solution {
    // pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    // }

    /*
    class Solution {
    HashMap<String, Integer> counts = new HashMap();
    List<TreeNode> list = new ArrayList();
    public List<TreeNode> findDuplicateSubtrees(TreeNode root) {
        dfs(root);
        return list;
    }

    private String dfs(TreeNode root){
        if (root == null) return "$";
        String res = root.val + ",";
        String left = dfs(root.left);
        String right = dfs(root.right);
        res += left + right;
        if (!res.equals("")){
            counts.put(res,counts.getOrDefault(res,0) + 1);
            if (counts.get(res) == 2){
                list.add(root);
            }
        }
        return res;
    }
}
     */
}