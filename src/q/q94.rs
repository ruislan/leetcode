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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 递归方法
        //  fn in_order(node: Node) {
        //       in_order(node.left);
        //       process(node);
        //       in_order(node.right);
        //  }
        // Passed 0ms 2mb
        // fn in_order(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        //     if let Some(mut node) = node {
        //         in_order(node.borrow_mut().left.take(), arr);
        //         arr.push(node.borrow().val);
        //         in_order(node.borrow_mut().right.take(), arr);
        //     }
        // }
        // let mut arr = Vec::new();
        // in_order(root, &mut arr);
        // arr

        // 方法2
        // 迭代方法
        //  while !stack.is_empty() {
        //       node = stack.peek();
        //       if node.left {
        //          stack.push(node.left);
        //          continue;
        //       }
        //       node = stack.pop();
        //       process(node);
        //       stack.push(node.right);
        //  }
        // Passed 0ms 2mb
        let mut arr = Vec::new();
        let mut stack = vec![root];
        while let Some(node) = stack.last_mut() {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                if left.is_some() {
                    stack.push(left);
                    continue;
                }
            }
            if let Some(node) = stack.pop().unwrap() {
                arr.push(node.borrow().val);
                stack.push(node.borrow_mut().right.take());
            }
        }
        arr
    }
}


#[test]
fn test() {
    assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })))), vec![1]);
    assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })))), vec![1, 2, 3]);
}