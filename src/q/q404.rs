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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 广度优先，根节点的值设置为0，右节点的值设置为0
        // 剩下的就都是左子节点了，加上之后就是结果
        // Passed 0ms 2.2mb
        // let mut res = 0;
        // let mut root = root;
        // let mut queue = std::collections::VecDeque::new();
        // if root.is_some() {
        //     root.as_mut().unwrap().borrow_mut().val = 0;
        //     queue.push_back(root);
        // }
        // while !queue.is_empty() {
        //     for _ in 0..queue.len() {
        //         let node = queue.pop_front().unwrap();
        //         let mut node = node.unwrap();
        //         let left = node.borrow_mut().left.take();
        //         let mut right = node.borrow_mut().right.take();
        //         if left.is_none() && right.is_none() { res += node.borrow().val; };
        //         if left.is_some() { queue.push_back(left); }
        //         if right.is_some() {
        //             right.as_mut().unwrap().borrow_mut().val = 0;
        //             queue.push_back(right);
        //         }
        //     }
        // }
        // res

        // 方法2
        // 前序遍历，如果当前处理的节点左节点是否是子节点，是就返回结果，否则返回0
        // Passed 0ms 2.1mb
        let mut sum = 0;
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if left.is_some() &&
                left.as_ref().unwrap().borrow().left.is_none() &&
                left.as_ref().unwrap().borrow().right.is_none() {
                sum += left.as_ref().unwrap().borrow().val
            }
            sum += Self::sum_of_left_leaves(left) + Self::sum_of_left_leaves(right);
        }
        sum
    }
}