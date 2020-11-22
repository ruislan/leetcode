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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 广度优先遍历子树，循环方式
        // Passed 0ms 2.5mb
        // match root {
        //     None => 0,
        //     Some(node) => {
        //         let (mut depth, mut arr) = (0, vec![node]);
        //         while !arr.is_empty() {
        //             let mut children = vec![];
        //             for mut node in arr {
        //                 let mut node = node.borrow_mut();
        //                 if let Some(left) = node.left.take() {
        //                     children.push(left);
        //                 }
        //                 if let Some(right) = node.right.take() {
        //                     children.push(right);
        //                 }
        //             }
        //             depth += 1;
        //             arr = children;
        //         }
        //         depth
        //     }
        // }

        // 方法2
        // 广度优先遍历子树，递归方式，取左右子节点最大的那个值
        // Passed 0ms 2.8mb
        match root {
            None => 0,
            Some(node) => {
                let mut node = node.borrow_mut();
                1 + Self::max_depth(node.left.take()).max(Self::max_depth(node.right.take()))
            }
        }
    }
}