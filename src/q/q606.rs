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
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        // 方法1
        //      1
        //    2   3
        //  4
        // 前序遍历的方式即 1 (2 (4)) (3)
        // 可以看出，当前节点的左节点和右节点都用括号括起来，如果是空就不用括号
        // 只需要注意一下左子节点为空，右子节点不为空的时候，需要保留左子的括号
        // Passed 0ms 2.5mb
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, res: &mut String) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                res.push_str(&node.borrow().val.to_string());
                match (left, right) {
                    (Some(left), Some(right)) => {
                        res.push('(');
                        pre_order(Some(left), res);
                        res.push_str(")(");
                        pre_order(Some(right), res);
                        res.push(')');
                    }
                    (Some(left), None) => {
                        res.push('(');
                        pre_order(Some(left), res);
                        res.push(')');
                    }
                    (None, Some(right)) => {
                        res.push_str("()(");
                        pre_order(Some(right), res);
                        res.push(')');
                    }
                    _ => ()
                }
            }
        }
        let mut res = String::new();
        pre_order(t, &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })))), "1".to_string());
    assert_eq!(Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: None,
    })))), "1(2)".to_string());
    assert_eq!(Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        left: None,
    })))), "1()(2)".to_string());
    assert_eq!(Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })))), "1(2)(3)".to_string());
}