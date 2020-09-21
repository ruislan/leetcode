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
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 中序遍历的镜像应用，每次计算都增加累加值
        // 本题与q538一样，就不多说了
        // Passed 0ms 1.9mb
        fn in_order_rev(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = node {
                in_order_rev(node.borrow().right.clone(), sum);
                node.borrow_mut().val += *sum;
                *sum = node.borrow().val;
                in_order_rev(node.borrow().left.clone(), sum)
            }
        }
        in_order_rev(root.clone(), &mut 0);
        root
    }
}