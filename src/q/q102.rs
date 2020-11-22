use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // 方法1
        // 按照广度优先遍历，然后收集所有的值即可
        // Passed 0ms 2.1mb
        let (root, mut queue, mut res) = (root, std::collections::VecDeque::new(), Vec::new());
        if root.is_some() { queue.push_back(root); }
        while !queue.is_empty() {
            let mut values = Vec::new();
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap().unwrap();
                values.push(node.borrow().val);

                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if left.is_some() { queue.push_back(left); }
                if right.is_some() { queue.push_back(right); }
            }
            res.push(values);
        }
        res
    }
}