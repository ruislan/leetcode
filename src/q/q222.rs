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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 方法1
        // 广度优先直接统计个数
        // Passed 4ms 4.5mb 时间O(N) 空间O(1)
        // let mut answer = 0;
        // let mut que = std::collections::VecDeque::new();
        // if root.is_some() { que.push_back(root); }
        // while !que.is_empty() {
        //     answer += que.len() as i32;
        //     for _ in 0..que.len() {
        //         let node = que.pop_front().unwrap().unwrap();
        //         let left = node.borrow_mut().left.take();
        //         let right = node.borrow_mut().right.take();
        //         if left.is_some() { que.push_back(left); }
        //         if right.is_some() { que.push_back(right); }
        //     }
        // }
        // answer

        // 方法2
        // 注意到完全二叉树的特点
        // 完全二叉树的定义如下：在完全二叉树中，除了最底层节点可能没填满外，
        // 其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。
        // 若最底层为第h层，则该层包含 1~2^h个节点。
        // 我们利用完全二叉树的特点，左叶子节点就是最大的高度，所以我们比较左右两个子树的高度
        // left_h和right_h：
        //       left_h == right_h:
        //          说明没有填满的叶子节点在右子树，将右子树的两个节点继续如此递归操作
        //       left_h > right_h：
        //          说明没有填满的叶子节点在左子树，将左子树的两个节点继续如此递归操作
        // Passed 4ms 4.5mb 时间O((logN)^2) 空间O(1)
        fn get_height(mut node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut h = 0;
            while let Some(ptr) = node {
                h += 1;
                node = ptr.borrow().left.clone();
            }
            h
        }

        fn count(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    let left_h = get_height(node.borrow().left.clone());
                    let right_h = get_height(node.borrow().right.clone());
                    if left_h == right_h {
                        count(node.borrow().right.clone()) + (1 << left_h)
                    } else {
                        count(node.borrow().left.clone()) + (1 << right_h)
                    }
                }
                None => 0
            }
        }

        count(root)
    }
}