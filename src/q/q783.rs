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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 关键词：BST，两点差的最小值
        // 意味着：中序遍历就是递增顺序
        // 问题转换成递增数组中两点差的最小值，递增数组的相邻两点的差值是最小的
        // 方法1
        // 中序遍历放入数组，然后直接求出前后两点差值，并找出最小值
        // Passed 0ms 2mb
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = node {
                in_order(node.borrow_mut().left.take(), nums);
                nums.push(node.borrow().val);
                in_order(node.borrow_mut().right.take(), nums);
            }
        }
        let mut nums = Vec::new();
        in_order(root, &mut nums);
        (1..nums.len()).map(|x| nums[x] - nums[x - 1]).min().unwrap_or(0)
    }
}