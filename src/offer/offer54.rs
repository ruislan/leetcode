use std::cell::RefCell;
use std::rc::Rc;

use crate::offer::Solution;

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
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // 方法1
        // 遍历所有的值，然后大到小排序，取第k - 1个值
        // let (mut root, mut queue, mut values) = (root, std::collections::VecDeque::new(), vec![]);
        // queue.push_back(root);
        // while !queue.is_empty() {
        //     (0..queue.len()).for_each(|_| {
        //         if let Some(node) = queue.pop_front().unwrap() {
        //             let mut node = node.borrow_mut();
        //             values.push(node.val);
        //             queue.push_back(node.left.take());
        //             queue.push_back(node.right.take());
        //         }
        //     });
        // }
        // values.sort_unstable_by(|a, b| b.cmp(&a));
        // values[k as usize - 1]

        // 方法2
        // 注意题目中提到的是二叉搜索树，即意味着左父右的顺序从小到大排列的
        // 那么我们按照这个顺序的倒序遍历之后的结果，取 k 即可
        // 要采用中序遍历的话，递归实际上是最简单的
        fn mfs(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            let node = node.as_ref().unwrap().borrow();
            if node.right.is_some() { mfs(node.right.clone(), values); }
            values.push(node.val);
            if node.left.is_some() { mfs(node.left.clone(), values); }
        }

        let mut values = vec![];
        mfs(root, &mut values);
        values[k as usize - 1]
    }
}