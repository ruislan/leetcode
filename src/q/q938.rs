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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        // 方法1
        // 中序遍历，如果当前值在[l..r]之间的相加即可
        // 递归
        // fn in_order(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, l: i32, r: i32) {
        //     if let Some(node) = node {
        //         in_order(node.borrow_mut().left.take(), sum, l, r);
        //         let val = node.borrow().val;
        //         if val <= r && val >= l { *sum += val; }
        //         in_order(node.borrow_mut().right.take(), sum, l, r);
        //     }
        // }
        // let mut sum = 0;
        // in_order(root, &mut sum, l, r);
        // sum

        // 方法1
        // 中序非递归版本
        // Passed 20ms 4.1mb
        // let mut sum = 0;
        // let mut stack = vec![root];
        // while !stack.is_empty() {
        //     if let Some(node) = stack.pop() {
        //         if let Some(node) = node {
        //             let left = node.borrow_mut().left.take();
        //             if left.is_some() {
        //                 stack.push(Some(node));
        //                 stack.push(left);
        //             } else {
        //                 let val = node.borrow().val;
        //                 if val <= r && val >= l { sum += val; }
        //                 stack.push(node.borrow_mut().right.take());
        //             }
        //         }
        //     }
        // }
        // sum

        // 方法2
        // 剪枝计算，因为BST，那么如果节点小于l，那么都不用去计算左边，如果节点大于r，那么它右边的都不用去计算了。
        // Passed 16ms 4.2mb
        let mut stack = vec![root];
        let mut sum = 0;
        while !stack.is_empty() {
            if let Some(node) = stack.pop() {
                if let Some(node) = node {
                    let val = node.borrow().val;
                    if val >= l && val <= r { sum += val; }
                    if val > l { stack.push(node.borrow_mut().left.take()); }
                    if val < r { stack.push(node.borrow_mut().right.take()); }
                }
            }
        }
        sum
    }
}