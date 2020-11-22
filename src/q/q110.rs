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

// 与offer55_ii相同
#[allow(unused)]
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 按层遍历树，
        // 计算每个节点的左右子节点高度差，相差超过1，返回false
        // 遍历结束，返回true
        // Passed 4ms 2.6mb
        // if root.is_none() { return true; }
        // let mut queue = std::collections::VecDeque::new();
        // queue.push_back(root.clone());
        // while !queue.is_empty() {
        //     for _ in 0..queue.len() {
        //         let mut node = queue.pop_front().unwrap().unwrap();
        //         let mut left = node.borrow().left.clone();
        //         let mut right = node.borrow().right.clone();
        //         if (Self::depth(left.clone()) - Self::depth(right.clone())).abs() > 1 { return false; }
        //         if left.is_some() { queue.push_back(left.clone()); }
        //         if right.is_some() { queue.push_back(right.clone()); }
        //     }
        // }
        // true

        // 方法2
        // 方法1的递归版本
        // Passed 0ms 2.7mb
        // match root {
        //     None => true,
        //     Some(node) => {
        //         (Self::depth(node.borrow().left.clone()) - Self::depth(node.borrow().right.clone())).abs() < 2 &&
        //             Self::is_balanced(node.borrow().left.clone()) &&
        //             Self::is_balanced(node.borrow().right.clone())
        //     }
        // }

        // 方法3
        // 方法1，2都重复计算了高度，如果我们能记录高度的话，就能减少很多计算
        // 这个计算实际上会传递高度，所以我们和方法1一样，额外写一个深度的函数
        // 我们核心就是用后续遍历的方式来遍历，
        // 这样能够保证一个父节点的所有子节点都能够被计算到，然后再来看父节点
        // post_order(node) {
        //    post_order(node.left)
        //    post_order(node.right)
        //    node.process()
        // }
        fn post_order(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    let left = post_order(node.borrow().left.clone());
                    let right = post_order(node.borrow().right.clone());
                    if (left - right).abs() > 1 || left == -1 || right == -1 {
                        -1
                    } else {
                        1 + left.max(right)
                    }
                }
            }
        }
        post_order(root) > -1
    }

    // 方法1和2的辅助函数
    // fn depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     match node {
    //         None => 0,
    //         Some(node) => {
    //             1 + Self::depth(node.borrow().left.clone()).max(Self::depth(node.borrow().right.clone()))
    //         }
    //     }
    // }
}