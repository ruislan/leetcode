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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 广度优先，直接交换左右子节点，然后把左右子节点放入queue，直到所有的叶子节点都处理结束
        // Passed 0ms 1.9mb
        root.map(|root| {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.clone());
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let mut node = node.borrow_mut();

                    let mut left = node.left.take();
                    node.left = node.right.take();
                    node.right = left;

                    if let Some(left) = node.left.clone() { queue.push_back(left); }
                    if let Some(right) = node.right.clone() { queue.push_back(right); }
                }
            }
            root
        })
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::invert_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))));

    assert_eq!(
        Solution::invert_tree(
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
            })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        }))));
}