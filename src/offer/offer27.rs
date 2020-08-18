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
use crate::offer::Solution;

impl Solution {
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 用递归处理
        // 递归的基本法则就是，1、找出基线；2、缩小范围
        // 这里的基线就是
        // 1.子节点都是空(left == None && right == None)的时候，就不需要翻转了
        // 2.子节点只要有一个不为空，就交换swap(left,right);
        // 缩小范围就是
        // 1.子节点不为空的，就放入递归去处理
        // 最后翻转完了之后，返回头结点

        // 方法2
        // 使用循环处理
        // 将要处理的节点放入stack，stack.push(root.as_mut());
        // 循环取出stack的节点直到stack为空
        // 1. 该节点的子节点都是空，不处理
        // 2. 该节点的子节点有一个不为空，就交换swap(left,right)
        // 将不为空的那个子节点放入stack
        let mut root = root;
        root
    }
}