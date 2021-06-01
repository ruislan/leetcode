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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        if root.is_none() { return None; }
        if root == p || root == q { return root; }
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        let left_parent = Self::lowest_common_ancestor(left, p.clone(), q.clone());
        let right_parent = Self::lowest_common_ancestor(right, p.clone(), q.clone());
        if left_parent.is_none() { return right_parent; }
        if right_parent.is_none() { return left_parent; }
        root
    }
}