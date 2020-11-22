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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 前序遍历，传输一个数字用于存储当前节点组合成的二进制数的数字
        // 当这个节点是叶子节点的时候，将这个数字纳入求和
        let mut sum = 0;
        let mut stack = vec![(root, 0)];
        while !stack.is_empty() {
            if let Some(item) = stack.pop() {
                if let Some(node) = item.0 {
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    let val = node.borrow().val;
                    let num = ((item.1 << 1) + val) % 1000000007;
                    if left.is_none() && right.is_none() {
                        sum += num;
                    } else {
                        stack.push((right, num));
                        stack.push((left, num));
                    }
                }
            }
        }
        sum
    }
}