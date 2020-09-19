use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

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

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        // 方法1
        // 广度优先吧，queue中存储节点和根结点到当前节点的所有点vec，当节点左右都是None，则直接记录
        // Passed 0ms 2.1mb
        // let mut queue = std::collections::VecDeque::new();
        // let mut res = Vec::new();
        // if root.is_some() { queue.push_back((root, String::new())); };
        // while !queue.is_empty() {
        //     for _ in 0..queue.len() {
        //         let (node, path) = queue.pop_front().unwrap();
        //         let mut node = node.unwrap();
        //         let left = node.borrow_mut().left.take();
        //         let right = node.borrow_mut().right.take();
        //         let path = path + node.borrow().val.to_string().as_str();
        //         if left.is_none() && right.is_none() { res.push(path.clone()); }
        //         if left.is_some() { queue.push_back((left, path.clone() + "->")); }
        //         if right.is_some() { queue.push_back((right, path.clone() + "->")); }
        //     }
        // }
        // res

        // 方法2
        // 来个前序遍历吧
        // Passed 0ms 2mb
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, path: String, res: &mut Vec<String>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                let path = path + node.borrow().val.to_string().as_str();
                if left.is_none() && right.is_none() { res.push(path.clone()); }
                if left.is_some() { pre_order(left, path.clone() + "->", res); }
                if right.is_some() { pre_order(right, path.clone() + "->", res); }
            }
        }
        let mut res = Vec::new();
        pre_order(root, String::new(), &mut res);
        res
    }
}