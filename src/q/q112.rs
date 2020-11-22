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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        // 方法1
        // 广度优先，queue中存储是节点和根节点到当前节点的和(node, path_sum)
        // 当节点没有左右子节点（叶子节点），查看path_sum是否与sum相同，相同则返回true
        // 0ms 2.4mb
        match root {
            None => false,
            Some(root) => {
                let mut queue = std::collections::VecDeque::new();
                queue.push_back((root, 0));
                while !queue.is_empty() {
                    for _ in 0..queue.len() {
                        let (node, mut path_sum) = queue.pop_front().unwrap();
                        path_sum += node.borrow().val;
                        let left = node.borrow_mut().left.take();
                        let right = node.borrow_mut().right.take();

                        if left.is_none() && right.is_none() && path_sum == sum { return true; }
                        if left.is_some() { queue.push_back((left.unwrap(), path_sum)); }
                        if right.is_some() { queue.push_back((right.unwrap(), path_sum)); }
                    }
                }
                false
            }
        }
    }
}