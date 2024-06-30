//binary tree
use std::cmp;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {

    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    
    let mut root = TreeNode::new(3);
    let mut node9 = TreeNode::new(9);
    let mut node20 = TreeNode::new(20);
    let mut node15 = TreeNode::new(15);
    let mut node7 = TreeNode::new(7);

    node20.left = Some(Box::new(node15));
    node20.right = Some(Box::new(node7));

    root.left = Some(Box::new(node9));
    root.right = Some(Box::new(node20));

    let depth = max_depth(Some(&Box::new(root)));
    println!("Maximum : {}", depth); 
}
