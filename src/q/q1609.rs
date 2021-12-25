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

use std::rc::Rc;
use std::cell::RefCell;
use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 层序遍历，按题目说明检查即可
        // AC 24ms 11.5mb 105/105
        use std::collections::VecDeque;
        let mut que = VecDeque::new();
        que.push_back(root);
        let mut level = 0;
        while !que.is_empty() {
            let size = que.len();
            let mut values = Vec::new();
            for _ in 0..size {
                let node = que.pop_front().unwrap();
                if let Some(node) = node {
                    let mut node_ptr_mut = node.borrow_mut();
                    que.push_back(node_ptr_mut.left.take());
                    que.push_back(node_ptr_mut.right.take());
                    values.push(node_ptr_mut.val);
                }
            }
            if level & 1 == 1 {
                if values.len() > 0 && values[0] & 1 == 1 { return false; }
                for i in 1..values.len() {
                    if values[i] >= values[i - 1] || values[i] & 1 == 1 { return false; }
                }
            } else {
                if values.len() > 0 && values[0] & 1 == 0 { return false; }
                for i in 1..values.len() {
                    if values[i] <= values[i - 1]  || values[i] & 1 == 0 { return false; }
                }
            }
            level += 1;
        }
        true
    }
}