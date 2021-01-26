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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 方法1
        // 前中后序遍历树都可
        // 然后合并数组
        // 然后排序合并后的数组
        // O(nlogn)

        // 方法2
        // 中序遍历树，这样保证遍历后的数组是有序的
        // 然后合并两个有序数组
        // 这里可以看作是归并排序的归并部分
        // O(m + n)
        // Passed 36ms 3.3mb
        fn in_order(mut node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                in_order(left, arr);
                arr.push(node.borrow().val);
                in_order(right, arr);
            }
        }

        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        in_order(root1, &mut v1);
        in_order(root2, &mut v2);
        let n = v1.len() + v2.len();
        let mut answer = vec![0; n];
        let mut i = 0;
        let mut j = 0;
        for k in 0..n {
            if i >= v1.len() {
                answer[k] = v2[j];
                j += 1;
            } else if j >= v2.len() {
                answer[k] = v1[i];
                i += 1;
            } else if v1[i] > v2[j] {
                answer[k] = v2[j];
                j += 1;
            } else {
                answer[k] = v1[i];
                i += 1;
            }
        }
        answer
    }
}