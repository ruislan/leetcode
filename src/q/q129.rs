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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 广度优先，遍历到所有叶子节点，然后将和累加起来
        // Passed 0ms 2.2mb
        // match root {
        //     None => 0,
        //     Some(mut root) => {
        //         let mut sum = 0;
        //         let mut queue = std::collections::VecDeque::new();
        //         queue.push_back((root, 0));
        //         while !queue.is_empty() {
        //             for _ in 0..queue.len() {
        //                 let (mut node, mut path_sum) = queue.pop_front().unwrap();
        //                 path_sum = path_sum * 10 + node.borrow().val;
        //                 let left = node.borrow_mut().left.take();
        //                 let right = node.borrow_mut().right.take();
        //                 if left.is_none() && right.is_none() { sum += path_sum; }
        //                 if left.is_some() { queue.push_back((left.unwrap(), path_sum)); }
        //                 if right.is_some() { queue.push_back((right.unwrap(), path_sum)); }
        //             }
        //         }
        //         sum
        //     }
        // }

        // 方法2
        // 前序遍历
        // Passed 0ms 2.1mb
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    sum = sum * 10 + node.borrow().val;
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_none() && right.is_none() {
                        sum
                    } else {
                        pre_order(left, sum) + pre_order(right, sum)
                    }
                }
            }
        }
        pre_order(root, 0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })))), 1);
    assert_eq!(Solution::sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })))), 21 + 23);
}