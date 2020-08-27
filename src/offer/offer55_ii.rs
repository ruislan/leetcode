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
use crate::offer::Solution;

// 与q110相同
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 方法1
        // 一句话来说就是要想平衡，当前层的上一层必须是满编的，只要不满，当前层还有下一层，必然不平衡。
        // 创建变量min表示至少没有一个子节点的那个节点的深度，初始为None
        // 创建depth表示树的当前深度
        // 按照广度优先遍历树，
        // 每遍历一层depth += 1，
        // 只要出现有一个节点不是完整两个子节点的且min还没赋值的，将这个depth赋值到min
        // 当min有值（Some），且depth - min > 1，直接返回false,
        // 迭代结束，返回true
        false
    }
}