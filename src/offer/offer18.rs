use crate::offer::Solution;
use std::borrow::{BorrowMut, Borrow};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 删除某一个节点需要线性查找节点的值与val相同的节点，当找到这个节点之后
        // 将这个节点的prev节点的next指向这个节点的next
        // 注意如果删除第一个节点，那么将第一个节点的下一个节点作为头结点即可
        // 如果删除最后一个节点，那么只需要将最后一个节点的前一个节点的next指向None即可
        // let mut head = head.clone();
        // let mut prev: Option<Box<ListNode>> = None;
        // let mut cur = head.clone();
        // while let Some(node) = &cur {
        //     if node.val == val {
        //         if prev == None {
        //             head = cur.clone().unwrap().next;
        //         } else {
        //             prev.clone().unwrap().next = node.next.clone();
        //         }
        //         break;
        //     }
        //     prev = cur;
        //     cur = prev.clone().unwrap().next.clone();
        // }
        head
    }
}