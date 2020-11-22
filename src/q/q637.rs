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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        // 方法1
        // 广度优先，一层一层的计算平均值即可
        let (mut res, mut queue) = (Vec::new(), std::collections::VecDeque::new());
        if root.is_some() { queue.push_back(root); }
        while !queue.is_empty() {
            let (mut sum, count) = (0_f64, queue.len() as f64);
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap().unwrap();
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if left.is_some() { queue.push_back(left); }
                if right.is_some() { queue.push_back(right); }
                sum += node.borrow().val as f64;
            }
            res.push(sum / count);
        }
        res
    }
}