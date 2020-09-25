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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // BST的特点就是，比当前值小的在左边，比当前值大的在右边
        // 我们按照这个思路去遍历树，比较当前节点的值与val的大小
        // 如果大，取当前节点的左子节点，继续
        // 如果小，取当前节点的右子节点，继续
        // 如果相等，返回当前节点
        // 如果当前节点为空，说明我们已经无法继续了，退出循环，返回空
        let mut node = root;

        while let Some(cur) = node {
            let cur_val = cur.borrow().val;
            match cur_val.cmp(&val) {
                std::cmp::Ordering::Equal => return Some(cur),
                std::cmp::Ordering::Greater => node = cur.borrow_mut().left.take(),
                std::cmp::Ordering::Less => node = cur.borrow_mut().right.take(),
            }
        }
        None
    }
}