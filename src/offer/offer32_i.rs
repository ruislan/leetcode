use std::cell::RefCell;
use std::rc::Rc;

use crate::offer::Solution;

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
    pub fn level_order_i(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 创建一个res作为返回的容器
        // 创建一个arr用来存储每层的节点，按照从上到下，从左到右的顺序依次访问root
        // 将每个节点的值存储到res
        // Passed 0ms 2.1mb
        let (mut res, mut arr) = (Vec::new(), Vec::new());
        if root.is_some() { arr.push(root); }
        while !arr.is_empty() {
            let mut children = Vec::new();
            for mut node in arr {
                let mut node = node.as_mut().unwrap().borrow_mut();
                res.push(node.val);
                if node.left.is_some() { children.push(node.left.take()); }
                if node.right.is_some() { children.push(node.right.take()); }
            }
            arr = children;
        }
        res

        // queue还是array？对于Rust来说，没有那么重要
        // Passed 0ms 2.2mb
        // let (mut root, mut res, mut queue) = (root, Vec::new(), std::collections::VecDeque::new());
        // if root.is_some() { queue.push_back(root); }
        // while !queue.is_empty() {
        //     for _ in 0..queue.len() {
        //         let mut node = queue.pop_front().unwrap();
        //         let mut node = node.as_mut().unwrap().borrow_mut();
        //         res.push(node.val);
        //         if node.left.is_some() { queue.push_back(node.left.take()); }
        //         if node.right.is_some() { queue.push_back(node.right.take()); }
        //     }
        // }
        // res
    }
}