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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 特点：坡度 = (左子树的所有节点的值得和 - 右子树的所有节点的值得和) 的绝对值，树的坡度就是所有节点的坡度和
        // 方法1
        // 递归，后序遍历的方式，这样可以先求出左右子树的数据，方便数据归集
        // 为了方便计算，我们需要将当前节点的值 设置为 左右子树的和与当前节点的值相加，
        // 这样相当于当前节点的值就是其所有子节点的和
        // 每次递归返回的就是当前节点和其所有子节点的坡度的和，简称当前整个子树的坡度
        // 伪代码
        // if root != null:
        //    left_tilt = find_tilt(root.left)
        //    right_tilt = find_tilt(root.right)
        //    left_val = if root.left != null { root.left.val } else { 0 }
        //    right_val = if root.right != null { root.right.val } else { 0 }
        //    root.val = root.val + left_val + right_val
        //    return left_tilt + right_tilt + (left_val - right_val).abs()
        //
        // Passed 4ms 2.6mb
        match root {
            None => 0,
            Some(node) => {
                let left_tilt = Self::find_tilt(node.borrow_mut().left.clone());
                let right_tilt = Self::find_tilt(node.borrow_mut().right.clone());
                let left_val = match node.borrow_mut().left.clone() {
                    None => 0,
                    Some(left) => left.borrow().val,
                };
                let right_val = match node.borrow_mut().right.clone() {
                    None => 0,
                    Some(right) => right.borrow().val,
                };
                node.borrow_mut().val += left_val + right_val;
                left_tilt + right_tilt + (left_val - right_val).abs()
            }
        }
    }
}