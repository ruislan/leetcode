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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 递归
        // Passed 0ms 1.9mb
        // fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        //     if let Some(node) = node {
        //         arr.push(node.borrow().val);
        //         pre_order(node.borrow_mut().left.take(), arr);
        //         pre_order(node.borrow_mut().right.take(), arr);
        //     }
        // }
        // let mut res = Vec::new();
        // pre_order(root, &mut res);
        // res

        // 方法2
        // 非递归，用栈，处理完当前节点后，将右子树先放入栈，然后将左子树后放入栈
        let mut res = Vec::new();
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                res.push(node.borrow().val);
                stack.push(node.borrow_mut().right.take());
                stack.push(node.borrow_mut().left.take());
            }
        }
        res
    }
}