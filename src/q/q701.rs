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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // BST首先找到要插入值的候选位置
        // val大于当前节点的值，就向右走，如果右边没有节点，就创建节点插入值
        // 小于当前节点的值就向左走，如果左边没有节点，就创建节点插入值
        // Passed 12ms 2.5mb
        // if root.is_none() { return Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))); }
        //
        // let mut node = root.clone();
        // while let Some(cur) = node {
        //     let cur_val = cur.borrow().val;
        //     let left = cur.borrow_mut().left.clone();
        //     let right = cur.borrow_mut().right.clone();
        //     if cur_val > val {
        //         if left.is_none() {
        //             cur.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: None })));
        //             node = None;
        //         } else {
        //             node = left;
        //         }
        //     } else {
        //         if right.is_none() {
        //             cur.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: None })));
        //             node = None;
        //         } else {
        //             node = right;
        //         }
        //     }
        // }
        // root

        // 方法2
        // 递归方式
        // 基线：节点为空，则造一个新的节点
        // 缩小范围： 一个节点，val大，则继续右子节点，val小，则继续左子节点，相等，则返回空
        // Passed 12ms 2.5mb
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))),
            Some(cur) => {
                let cur_val = cur.borrow().val;
                let left = cur.borrow().left.clone();
                let right = cur.borrow().right.clone();
                if cur_val > val {
                    cur.borrow_mut().left = Self::insert_into_bst(left, val);
                } else {
                    cur.borrow_mut().right = Self::insert_into_bst(right, val);
                }
                Some(cur)
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::insert_into_bst(None, 1), Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))));
    assert_eq!(Solution::insert_into_bst(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }))), 2),
               Some(Rc::new(RefCell::new(TreeNode {
                   val: 1,
                   left: None,
                   right: Some(Rc::new(RefCell::new(TreeNode {
                       val: 2,
                       left: None,
                       right: None,
                   }))),
               })))
    );
    assert_eq!(Solution::insert_into_bst(Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    }))), 2),
               Some(Rc::new(RefCell::new(TreeNode {
                   val: 5,
                   left: Some(Rc::new(RefCell::new(TreeNode {
                       val: 2,
                       left: None,
                       right: None,
                   }))),
                   right: None,
               })))
    );
}