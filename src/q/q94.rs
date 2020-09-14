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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 递归方法
        //  fn in_order(node: Node) {
        //       in_order(node.left);
        //       process(node);
        //       in_order(node.right);
        //   }


        // 方法2
        // 迭代方法
        //  while !stack.is_empty() {
        //       stack.push(node.left);
        //       process(stack.pop());
        //       stack.push(node.right);
        //   }
        vec![]
    }
}