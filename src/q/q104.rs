// Definition for a binary tree node.
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
use crate::q::Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 采用广度优先遍历，当所有的子节点都没有子节点的时候，返回深度
        let (mut depth, mut queue) = (0, std::collections::VecDeque::new());
        if root.is_some() { queue.push_back(root); }
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let mut node = queue.pop_front().unwrap();
                let mut node = node.as_mut().unwrap().borrow_mut();
                if node.left.is_some() { queue.push_back(node.left.take()); }
                if node.right.is_some() { queue.push_back(node.right.take()); }
            }
            depth += 1;
        }
        depth
    }
}