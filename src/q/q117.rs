#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::q::Solution;

impl Solution {
    // 没有Rust通道，采用kotlin进行提交
    pub fn connect(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 采用广度优先，然后让每个节点指向右边的节点即可
        // Kotlin code
        // Passed 224ms 33.3mb
//        val queue = java.util.ArrayDeque<Node>()
//        if (root != null) queue.add(root)
//        while (queue.isNotEmpty()) {
//            var prev: Node? = null
//            for (i in 0 until queue.size) {
//                val cur = queue.poll()
//                if (prev == null) prev = cur
//                else {
//                    prev.next = cur
//                    prev = prev.next
//                }
//                if (cur.left != null) queue.addLast(cur.left!!)
//                if (cur.right != null) queue.addLast(cur.right!!)
//            }
//        }
//        return root
        let mut queue = std::collections::VecDeque::new();
        if root.is_some() { queue.push_back(root.clone()); }
        while !queue.is_empty() {
            let mut prev = None;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                if prev.is_none() {
                    prev = node.clone();
                } else {
                    prev.as_mut().unwrap().borrow_mut().next = node.clone();
                    prev = prev.unwrap().borrow().next.clone();
                }
                let left = node.as_ref().unwrap().borrow().left.clone();
                let right = node.as_ref().unwrap().borrow().right.clone();
                if left.is_some() { queue.push_back(left); }
                if right.is_some() { queue.push_back(right); }
            }
        }
        root
    }
}