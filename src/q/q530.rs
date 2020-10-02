use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

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

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 注意是一个BST树，那么任意两个节点的差值的绝对值的最小值，
        // 实际上就是中序遍历的情况下，连续两个节点的差的绝对值的最小值
        // Passed 4ms 2.8mb
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                in_order(left, nums);
                nums.push(node.borrow().val);
                in_order(right, nums);
            }
        }
        let mut nums = Vec::new();
        in_order(root, &mut nums);
        nums.windows(2).map(|x| (x[1] - x[0]).abs()).min().unwrap()
    }
}