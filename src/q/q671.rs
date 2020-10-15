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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 特点：非空，都是正数，子节点要么都有，要么没有（2或者0），节点的值等于其两个子节点中较小的那个
        // 意味着： 至少有一个节点；两个子节点的值有可能一样大；层数越深，节点的值越大，根节点的值永远是最小的（其中一个）；
        // 方法1
        // 只需要判断根节点的两个子节点，如果子节点存在且不相等，那么直接取值大的那个即可
        0
    }
}