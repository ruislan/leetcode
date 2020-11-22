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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 后序的意思就是父节点最后输出，左右父，递归很容易完成
        // Passed 0ms 2mb
        // fn post_order(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        //     if let Some(node) = node {
        //         post_order(node.borrow_mut().left.take(), values);
        //         post_order(node.borrow_mut().right.take(), values);
        //         values.push(node.borrow().val);
        //     }
        // }
        // let mut res = Vec::new();
        // post_order(root, &mut res);
        // res

        // 方法2
        // 非递归方式，用stack进行处理，后序的难点在于父节点是最后处理的
        // 假设树结构为：
        //               1
        //             2   3
        //           4  5 6  8
        //  前序为： 1 2 4 5 3 6 8
        //  中序为： 4 2 5 1 6 3 8
        //  后序为： 4 5 2 6 8 3 1
        //  把后序翻转一下
        //  翻后序： 1 3 8 6 2 5 4
        //  嗯，看起来是前序的变种，前序先向右走
        // Passed 0ms 2mb
        match root {
            None => Vec::new(),
            Some(node) => {
                let (mut res, mut stack) = (Vec::new(), vec![node]);
                while !stack.is_empty() {
                    let node = stack.pop().unwrap();
                    res.push(node.borrow().val);
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if let Some(left) = left { stack.push(left); }
                    if let Some(right) = right { stack.push(right); }
                }
                res.reverse();
                res
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::postorder_traversal(None), vec![]);
    assert_eq!(Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })))), vec![1]);
    assert_eq!(Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
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
    })))), vec![1, 3, 2]);
}