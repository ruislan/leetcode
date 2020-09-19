use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    children: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl Solution {
    // 此题没有Rust的提交窗口，用kotlin解决
    // 方法max_depth与之前q109的方法冲突，更名为max_depth_n
    fn max_depth_n(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 递归，从叶子结点开始回溯，返回每层最大的深度的那个叶子结点 + 1
        // Kotlin Passed 228ms 35.9mb
        // fun maxDepth(root: Node?): Int {
        // return if (root != null) {
        // 1 + if (root.children.isNotEmpty()) root.children.map { maxDepth(it) }.max()!! else 0
        // } else 0
        // }
        // }
        0
    }
}