// Definition for a binary tree node.
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
use crate::Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 按照广度优先遍历树
        // 创建一个FIFO的队列queue，首先将两个root放进去，
        // 迭代queue，每次弹出两个节点，这两个节点是两棵树的对位节点(le, re)，比较两个节点的值
        // 如果都为None则继续迭代， 如果值不等或者有一个节点为None，则返回false
        // 将le.left和re.right与le.right和re.left作为两个对位节点放入queue中，持续迭代到queue为空为止
        // 迭代完返回true

        // 方法2
        // 用递归方式
        // 创建一个比较方法check，将两个root放入
        // 比较两个节点，如果两个节点都为None返回true，如果某个节点为None，返回false
        // 返回两个节点的值是否相等 以及 递归放入le.left和re.right与le.right和re.left到此check方法中递归返回的结果即可
        true
    }
}