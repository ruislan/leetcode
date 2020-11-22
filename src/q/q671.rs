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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 遍历节点放入数组，直接选出第二大的即可
        // Passed 0ms 1.9mb
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = node {
                nums.push(node.borrow().val);
                pre_order(node.borrow_mut().left.take(), nums);
                pre_order(node.borrow_mut().right.take(), nums);
            }
        }
        let mut nums = Vec::new();
        pre_order(root, &mut nums);
        nums.sort_unstable();
        nums.dedup();
        if nums.len() < 2 { -1 } else { nums[1] }
    }
}