// Given a binary tree, implement a function that returns the maximum depth of the tree.
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

use std::rc::Rc;
use std::cell::RefCell;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example tree:
    //     3
    //    / \
    //   9  20
    //     /  \
    //    15   7
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let depth = max_depth(root);
    println!("Maximum depth of the tree: {}", depth);
}
// complexity -> O(n)