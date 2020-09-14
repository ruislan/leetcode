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
use crate::q::Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 广度优先，queue中存储节点和根节点到当前节点的所有值(node, vec![])，
        // 当到达叶子节点的时候，vec![]中所有的值，若值等于sum，放入结果
        // Passed 0ms 2.6mb
        // match root {
        //     None => Vec::new(),
        //     Some(root) => {
        //         let mut res = Vec::new();
        //         let mut queue = std::collections::VecDeque::new();
        //         queue.push_back((root, Vec::new()));
        //         while !queue.is_empty() {
        //             for _ in 0..queue.len() {
        //                 let (mut node, mut path_values) = queue.pop_front().unwrap();
        //                 path_values.push(node.borrow().val);
        //
        //                 let left = node.borrow_mut().left.take();
        //                 let right = node.borrow_mut().right.take();
        //
        //                 if left.is_none() && right.is_none() && path_values.iter().sum::<i32>() == sum { res.push(path_values.clone()); }
        //                 if left.is_some() { queue.push_back((left.unwrap(), path_values.clone())); }
        //                 if right.is_some() { queue.push_back((right.unwrap(), path_values.clone())); }
        //             }
        //         }
        //         res
        //     }
        // }
        // 方法2
        // 前序遍历，用stack存储前序的值
        // Passed 0ms 2.4mb
        fn traverse(node: Option<Rc<RefCell<TreeNode>>>, sum: i32, path_values: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if let Some(node) = node {
                path_values.push(node.borrow().val);
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if left.is_none() && right.is_none() && sum == path_values.iter().sum::<i32>() {
                    res.push(path_values.clone());
                }
                traverse(left, sum, path_values, res);
                traverse(right, sum, path_values, res);
                path_values.pop();
            }
        }

        let mut res = Vec::new();
        traverse(root, sum, &mut Vec::new(), &mut res);
        res
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::path_sum(Some(Rc::new(RefCell::new(TreeNode {
    //     val: 1,
    //     left: None,
    //     right: None,
    // }))), 1), vec![vec![1]]);

    assert_eq!(Solution::path_sum(Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    }))), 3), vec![vec![1, 2], vec![1, 2]]);
}