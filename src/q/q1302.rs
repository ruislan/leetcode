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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // dfs遍历，记录每个节点的值和对应的val。
        // 如果有更深一层的节点，则把之前的节点都弃掉。
        // 如果是更浅一层的节点，则直接跳过。
        // AC 4ms 3mb
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: i32, v: &mut (i32, Vec<i32>)) {
            if let Some(node) = node {
                if level > v.0 {
                    v.0 = level;
                    v.1 = vec![node.borrow().val];
                } else if level == v.0 {
                    v.1.push(node.borrow().val);
                }
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                dfs(left, level + 1, v);
                dfs(right, level + 1, v);
            }
        }
        let mut answer = (0, Vec::new());
        dfs(root, 0, &mut answer);
        answer.1.into_iter().sum()
    }
}