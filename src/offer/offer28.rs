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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // p1,p2两个指针分别指向root，p1从左边开始遍历树，p2从右边开始遍历树
        // 如果两个遍历的相同位置的值不一样，则返回False
        // 这里大着胆子clone，因为Rc的clone就是多了一个访问同一内存的Rc指针而已
        // 注意Option的clone实际就是内部的clone，而内部就是Rc，而Rc的clone和Rc::clone()是一个意思
        let mut stack = Vec::new();
        stack.push(root.clone());
        stack.push(root.clone());
        while !stack.is_empty() {
            let node1 = stack.pop().unwrap();
            let node2 = stack.pop().unwrap();

            if node1.is_none() && node2.is_none() { continue; }
            if node1.is_none() || node2.is_none() { return false; }

            let node1 = node1.unwrap();
            let node2 = node2.unwrap();

            if node1.borrow().val != node2.borrow().val { return false; }
            stack.push(node1.borrow().left.clone());
            stack.push(node2.borrow().right.clone());
            stack.push(node1.borrow().right.clone());
            stack.push(node2.borrow().left.clone());
        }
        true
    }
}