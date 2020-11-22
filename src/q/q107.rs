use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
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

#[allow(unused)]
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // 方法1
        // 这里其实用广度优先遍历即可，建立一个数组arr存储所有的节点，每层建立一个数组children存储该层所有的数组，
        // 最后翻转arr即可
        let (mut arr, mut res) = (Vec::new(), Vec::new());
        if root.is_some() { arr.push(root); }
        while !arr.is_empty() {
            let (mut children, mut values) = (Vec::new(), Vec::new());
            for mut node in arr {
                let mut node = node.as_mut().unwrap().borrow_mut();
                values.push(node.val);
                if node.left.is_some() { children.push(node.left.take()); }
                if node.right.is_some() { children.push(node.right.take()); }
            }
            arr = children;
            res.push(values);
        }
        res.into_iter().rev().collect()
    }
}