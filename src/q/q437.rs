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
    pub fn path_sum_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        // 方法1
        // dfs
        // 每次向下我们都将结果增加1位，然后遍历，如果结果正好等于target，就叠加计数
        // AC 0ms 2.2mb 126/126
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, target: i32, path: &mut Vec<i32>) -> i32 {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                let val = node.borrow().val;
                path.iter_mut().for_each(|x| *x += val);
                path.push(val);
                let count = path.iter().filter(|&&x| x == target).count() as i32;

                let left_count = dfs(left, target, path);
                let right_count = dfs(right, target, path);

                path.pop();
                path.iter_mut().for_each(|x| *x -= val);
                return left_count + right_count + count;
            }
            0
        }
        dfs(root, target_sum, &mut Vec::new())
    }
}