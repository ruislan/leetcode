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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        // 方法1
        // 广度优先搜索，这样每次都能访问统一深度的兄弟节点
        // 然后找出兄弟节点的儿子们是否是堂兄弟
        // 我们利用所有节点的值都不相同的特性，在每次迭代同一深度的兄弟节点时，创建一个vec
        // 如果该节点的子节点的值等于x或者y，就将该节点的值存入vec
        // 当这层迭代完成时，检查这个vec中的值：
        //    如果只有1个值，那么必然x和y不是堂兄弟，返回false
        //    如果有2个值：
        //       返回 第一个值（x或y的父节点）和 第二个值（x或y的父节点）是否相等
        //       相等则说明x和y时亲兄弟，不相等就是堂兄弟
        // Passed 0ms 2mb
        let mut que = std::collections::VecDeque::new();
        if root.is_some() { que.push_back(root); }
        while !que.is_empty() {
            let mut found = Vec::new();
            for _ in 0..que.len() {
                if let Some(node) = que.pop_front() {
                    if let Some(node) = node {
                        let left = node.borrow_mut().left.take();
                        let right = node.borrow_mut().right.take();
                        let parent_val = node.borrow().val;
                        if left.is_some() {
                            let val = left.as_ref().unwrap().borrow().val;
                            if val == x || val == y {
                                found.push(parent_val);
                            }
                            que.push_back(left);
                        }
                        if right.is_some() {
                            let val = right.as_ref().unwrap().borrow().val;
                            if val == x || val == y {
                                found.push(parent_val);
                            }
                            que.push_back(right);
                        }
                    }
                }
            }
            if !found.is_empty() {
                return
                    if found.len() == 1 {
                        false
                    } else {
                        found[0] != found[1]
                    };
            }
        }
        false
    }
}