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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 中序遍历，将所有的数字存入map中，然后从map中找出最大的频率的数字
        // 4ms 2.8mb
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut std::collections::HashMap<i32, i32>) {
            if let Some(node) = node {
                in_order(node.borrow_mut().left.take(), nums);
                let count = nums.entry(node.borrow().val).or_insert(0);
                *count += 1;
                in_order(node.borrow_mut().right.take(), nums);
            }
        }
        let mut map = std::collections::HashMap::new();
        in_order(root, &mut map);
        let mut res = Vec::new();
        let mut max = i32::min_value();
        for (num, count) in map {
            if count > max {
                max = count;
                res.clear();
                res.push(num);
            } else if count == max {
                res.push(num);
            }
        }
        res
    }
}