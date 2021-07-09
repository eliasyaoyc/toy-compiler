use std::rc::Rc;
use std::cell::RefCell;
use std::str::FromStr;

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
            right: None,
        }
    }
}

struct Codec {}

static null: &str = "#";

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        ser(root, &mut result);
        result.pop();
        result
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data == null {
            return None;
        }
        let v: Vec<&str> = data.split(',').collect();
        let mut root = newNode(v[0]);
        let mut index = 1;
        let root_ref = root.as_mut().unwrap();
        root_ref.borrow_mut().left = des(&v, &mut index);
        root_ref.borrow_mut().right = des(&v, &mut index);
        root
    }
}

fn newNode(v: &str) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(i32::from_str(v).unwrap()))))
}

fn ser(root: Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
    match root {
        None => {
            result.push('#');
            result.push(',');
        }
        Some(node) => {
            result.push_str(node.borrow().val.to_string().as_str());
            result.push(',');
            ser(node.borrow_mut().left.take(), result);
            ser(node.borrow_mut().right.take(), result);
        }
    }
}

fn des(v: &Vec<&str>, index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
    if v[*index] == null {
        *index += 1;
        return None;
    } else {
        let mut node = newNode(v[*index]);
        *index += 1;

        let mut node_ref = node.as_mut().unwrap();
        node_ref.borrow_mut().left = des(v, index);
        node_ref.borrow_mut().right = des(v, index);
        node
    }
}