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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 前序遍历，只要是叶子节点的，收集起来
        // 前序遍历是为了保证叶子节点的序列是从左到右的收集
        // Passed 0ms 2.2mb
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if left.is_none() && right.is_none() {
                    values.push(node.borrow().val);
                }
                pre_order(left, values);
                pre_order(right, values);
            }
        }
        let (mut values1, mut values2) = (Vec::new(), Vec::new());
        pre_order(root1, &mut values1);
        pre_order(root2, &mut values2);
        values1 == values2
    }
}