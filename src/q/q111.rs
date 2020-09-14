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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 创建变量depth存储深度，初始为0，
        // 按照广度优先遍历树，每次遍历时，深度加1，
        // 当出现某个节点左右子节点都没有的时候，返回这个深度
        match root {
            None => 0,
            Some(root) => {
                let (mut depth, mut arr) = (1, vec![root]);
                'found: while !arr.is_empty() {
                    let mut children = Vec::new();
                    for node in arr {
                        if node.borrow().left.is_none() && node.borrow().right.is_none() { break 'found; }
                        if node.borrow().left.is_some() { children.push(node.borrow().left.clone().unwrap()); }
                        if node.borrow().right.is_some() { children.push(node.borrow().right.clone().unwrap()); }
                    }
                    arr = children;
                    depth += 1;
                }
                depth
            }
        }
    }
}