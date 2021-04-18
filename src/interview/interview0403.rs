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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use crate::interview::Solution;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    pub fn list_of_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Box<ListNode>>> {
        // 方法1
        // 直接用层序遍历解决就行了
        // AC 0ms 1.9mb
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(tree);
        let mut answer = Vec::new();
        while !queue.is_empty() {
            let mut linked_list = Some(Box::new(ListNode { val: 0, next: None }));
            let mut ptr = linked_list.as_mut();
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front().unwrap() {
                    let val = node.borrow().val;
                    ptr.as_mut().unwrap().next = Some(Box::new(ListNode { val, next: None }));
                    ptr = ptr.unwrap().next.as_mut();

                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_some() { queue.push_back(left); }
                    if right.is_some() { queue.push_back(right); }
                }
            }
            answer.push(linked_list.unwrap().next.take());
        }
        answer
    }
}
