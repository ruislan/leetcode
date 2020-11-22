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
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 迭代s，当s.val == t.val
        // 如果s.val == t.val && is_sub(s.left, t.left) && is_sub(s.right, s.right)：
        //    则说明两个相等
        // 否则：
        //    继续下一个节点查看是否相等
        fn is_same(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (s, t) {
                (Some(s), Some(t)) =>
                    s.borrow().val == t.borrow().val &&
                        is_same(s.borrow().left.clone(), t.borrow().left.clone()) &&
                        is_same(s.borrow().right.clone(), t.borrow().right.clone()),
                (None, None) => true,
                _ => false
            }
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s);
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    if let Some(node) = node {
                        queue.push_back(node.borrow().left.clone());
                        queue.push_back(node.borrow().right.clone());
                        if is_same(Some(node), t.clone()) { return true; }
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_subtree(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
    ), true);
    assert_eq!(Solution::is_subtree(
        Some(Rc::new(RefCell::new(TreeNode {
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
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
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
        }))),
    ), true);
    assert_eq!(Solution::is_subtree(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        }))),
    ), true);
}