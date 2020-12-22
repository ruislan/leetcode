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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // 方法1
        // 按照广度遍历，将结果存储到数组中
        // 然后根据每层的奇偶看是否翻转结果数组即可
        // Passed 0ms 2.3 mb
        // let mut backward = false;
        // let mut que = std::collections::VecDeque::new();
        // if root.is_some() { que.push_back(root); }
        // let mut answer = Vec::new();
        // while !que.is_empty() {
        //     let mut values = Vec::new();
        //     for _ in 0..que.len() {
        //         if let Some(node) = que.pop_front().unwrap() {
        //             values.push(node.borrow().val);
        //             let left = node.borrow_mut().left.take();
        //             let right = node.borrow_mut().right.take();
        //             if left.is_some() { que.push_back(left); }
        //             if right.is_some() { que.push_back(right); }
        //         }
        //     }
        //     if backward { values.reverse(); }
        //     backward = !backward;
        //     answer.push(values);
        // }
        // answer

        // 方法2
        // 广度优先遍历，双端队列直接取就行了
        // Passed 0ms 2.2mb
        let mut forward = true;
        let mut que = std::collections::VecDeque::new();
        if root.is_some() { que.push_back(root); }
        let mut answer = Vec::new();
        while !que.is_empty() {
            let mut values = Vec::new();
            if forward {
                for _ in 0..que.len() {
                    if let Some(node) = que.pop_front().unwrap() {
                        values.push(node.borrow().val);
                        let left = node.borrow_mut().left.take();
                        let right = node.borrow_mut().right.take();
                        if left.is_some() { que.push_back(left); }
                        if right.is_some() { que.push_back(right); }
                    }
                }
            } else {
                for _ in 0..que.len() {
                    if let Some(node) = que.pop_back().unwrap() {
                        values.push(node.borrow().val);
                        let left = node.borrow_mut().left.take();
                        let right = node.borrow_mut().right.take();
                        if right.is_some() { que.push_front(right); }
                        if left.is_some() { que.push_front(left); }
                    }
                }
            }
            forward = !forward;
            answer.push(values);
        }
        answer
    }
}