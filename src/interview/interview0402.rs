// #[derive(Debug, PartialEq, Eq)]
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
use crate::interview::Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 将数组的中点作为根节点，数组的左边是其左子树，数组的右边是其右子树
        // 将左子树递归，将右子树递归，直到没有更多的数组
        // Passed 0ms 2.9mb
        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() { return None; }
            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: build(&nums[..mid]),
                right: build(&nums[mid + 1..]),
            })))
        }
        build(&nums)
    }
}