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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 中序遍历，将所有的树节点取出来
        // 然后按照顺序直接拼接即可
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                in_order(left, arr);
                arr.push(node);
                in_order(right, arr);
            }
        }
        let mut arr = Vec::new();
        in_order(root, &mut arr);
        let mut dummy = Rc::new(RefCell::new(TreeNode { val: 0, left: None, right: None }));
        let mut cur = dummy.clone();
        arr.into_iter().for_each(|node| {
            cur.borrow_mut().right = Some(node.clone());
            cur = node;
        });
        let root = dummy.borrow_mut().right.take();
        root
    }
}
