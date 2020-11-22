use std::cell::RefCell;
use std::rc::Rc;

use crate::interview::Solution;

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 用后续遍历法递归来计算深度，
        // 如果左右子树高度差为-1或者当左右子树高度差大于1，设置当前节点高度为-1，
        // 继续递归计算深度
        // post_order(node) {
        //    post_order(node.left);
        //    post_order(node.right);
        //    process node;
        // }
        fn post_order(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    let left = post_order(node.borrow().left.clone());
                    let right = post_order(node.borrow().right.clone());
                    if left == -1 || right == -1 || (left - right).abs() > 1 {
                        -1
                    } else {
                        1 + left.max(right)
                    }
                }
            }
        }
        post_order(root) != -1
    }
}