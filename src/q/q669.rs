use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

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

impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // 特点：BST，最小值，最大值
        // 意味着：左子节点的值都比父节点小，右子节点的值都比父节点大
        // 方法1
        // 递归方法，
        // 如果当前节点值在Low和High以内，表示此节点不用修剪，那么此节点的左子节点等于左子节点递归后的结果，右边类推
        // 如果当前节点值正好等于low，表示此节点的左子节点剪枝，此节点的右子节点等于右子节点递归后的结果，返回此节点
        // 如果当前节点值小于low，表示此节点和此节点的左子节点剪枝，返回右子节点的递归结果
        // 如果当前节点值正好等于high，表示此节点的右子节点剪枝，此节点的左子节点等于左子节点递归后的结果，返回此节点
        // 如果当前节点值大于high，表示此节点和此节点的右子节点剪枝，返回左子节点的递归结果
        // Passed 0ms 3.1mb
        match root {
            None => None,
            Some(node) => {
                let val = node.borrow().val;
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if val > low && val < high {
                    node.borrow_mut().left = Self::trim_bst(left, low, high);
                    node.borrow_mut().right = Self::trim_bst(right, low, high);
                    Some(node)
                } else if val == low {
                    node.borrow_mut().right = Self::trim_bst(right, low, high);
                    Some(node)
                } else if val < low {
                    Self::trim_bst(right, low, high)
                } else if val == high {
                    node.borrow_mut().left = Self::trim_bst(left, low, high);
                    Some(node)
                } else {
                    Self::trim_bst(left, low, high)
                }
            }
        }
    }
}