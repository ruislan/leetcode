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
    pub fn level_order_iii(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // 方法1
        // 创建一个数组arr用来缓存每层的节点，创建res用来存储每层的节点的值
        // 按照广度优先遍历节点，将节点的值存储在values数组中，并将values存储在res中
        // 迭代res，索引是奇数的values翻转
        // Passed 0ms 2.1mb
        // let (mut root, mut arr, mut res) = (root, Vec::new(), Vec::new());
        // if root.is_some() { arr.push(root); }
        // while !arr.is_empty() {
        //     let (mut children, mut values) = (Vec::new(), Vec::new());
        //     for mut node in arr {
        //         let mut node = node.as_mut().unwrap().borrow_mut();
        //         values.push(node.val);
        //         if node.left.is_some() { children.push(node.left.take()); }
        //         if node.right.is_some() { children.push(node.right.take()); }
        //     }
        //     res.push(values);
        //     arr = children;
        // }
        // res.into_iter().enumerate().map(|(i, mut v)| {
        //     if i & 1 == 1 { v.reverse(); }
        //     v
        // }).collect()

        // 方法2
        // 改进方法1，不用在最后reverse，创建一个是否翻转的标志rev，每次while循环，rev对自己取反
        // 如果rev为真，则翻转values，否则不用翻转
        // Passed 0ms 2.1mb
        let (mut arr, mut rev) = (Vec::new(), false);
        let mut res = Vec::new();
        if root.is_some() { arr.push(root); }
        while !arr.is_empty() {
            let (mut children, mut values) = (Vec::new(), Vec::new());
            for mut node in arr {
                let mut node = node.as_mut().unwrap().borrow_mut();
                values.push(node.val);
                if node.left.is_some() { children.push(node.left.take()); }
                if node.right.is_some() { children.push(node.right.take()); }
            }
            if rev { values.reverse(); };
            res.push(values);
            arr = children;
            rev = !rev;
        }
        res
    }
}