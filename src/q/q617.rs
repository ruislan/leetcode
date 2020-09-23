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
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 先比较t1和t2，如果t1和t2都不为空，继续，否则，返回不为空的那个或者都为空返回空
        // 广度优先，
        // 首先将t2的值添加到t1，然后比较t1和t2的左和t1和t2的右子节点，
        // 左或右会有4个情况
        // (none,none): 不处理，不放入queue
        // (左，none): 不处理，不放入queue
        // (none，左): 剪枝右边的到左边，不放入queue
        // (左，左)： 直接放入queue
        // 右边也一样
        // Passed 4ms 2.1mb
        // match (t1.clone(), t2.clone()) {
        //     (None, None) => None,
        //     (_, None) => t1,
        //     (None, _) => t2,
        //     _ => {
        //         let mut queue = std::collections::VecDeque::new();
        //         queue.push_back((t1.clone(), t2));
        //         while !queue.is_empty() {
        //             for _ in 0..queue.len() {
        //                 let (mut t1, mut t2) = queue.pop_front().unwrap();
        //                 t1.as_mut().unwrap().borrow_mut().val += t2.as_ref().unwrap().borrow().val;
        //                 let t1_left = t1.as_ref().unwrap().borrow().left.clone();
        //                 let t2_left = t2.as_mut().unwrap().borrow_mut().left.take();
        //                 let t1_right = t1.as_ref().unwrap().borrow().right.clone();
        //                 let t2_right = t2.as_mut().unwrap().borrow_mut().right.take();
        //                 match (t1_left, t2_left) {
        //                     (None, Some(t2_left)) => t1.as_mut().unwrap().borrow_mut().left = Some(t2_left),
        //                     (Some(t1_left), Some(t2_left)) => queue.push_back((Some(t1_left), Some(t2_left))),
        //                     _ => ()
        //                 }
        //                 match (t1_right, t2_right) {
        //                     (None, Some(t2_right)) => t1.as_mut().unwrap().borrow_mut().right = Some(t2_right),
        //                     (Some(t1_right), Some(t2_right)) => queue.push_back((Some(t1_right), Some(t2_right))),
        //                     _ => ()
        //                 }
        //             }
        //         }
        //         t1
        //     }
        // }

        // 方法2
        // 递归方法
        // Passed 8ms 2.1mb
        match (t1.clone(), t2.clone()) {
            (None, None) => None,
            (_, None) => t1,
            (None, _) => t2,
            _ => {
                let (mut t1, mut t2) = (t1, t2);
                t1.as_mut().unwrap().borrow_mut().val += t2.as_ref().unwrap().borrow().val;

                let t1_left = t1.as_ref().unwrap().borrow().left.clone();
                let t2_left = t2.as_mut().unwrap().borrow_mut().left.take();
                let t1_right = t1.as_ref().unwrap().borrow().right.clone();
                let t2_right = t2.as_mut().unwrap().borrow_mut().right.take();
                t1.as_mut().unwrap().borrow_mut().left = Self::merge_trees(t1_left, t2_left);
                t1.as_mut().unwrap().borrow_mut().right = Self::merge_trees(t1_right, t2_right);

                t1
            }
        }
    }
}