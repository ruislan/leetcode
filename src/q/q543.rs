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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 后续遍历，分别结算左右子树的高度，相加即为直径
        // 然后计算每个点的左右子树的高度，取最大的那个直径为结果
        // Passed 0ms 2.4mb
        fn height(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    let left_h = height(left, diameter);
                    let right_h = height(right, diameter);
                    *diameter = (left_h + right_h).max(*diameter);
                    1 + left_h.max(right_h)
                }
            }
        }
        let mut diameter = 0;
        height(root, &mut diameter);
        diameter
    }
}