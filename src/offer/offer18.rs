use crate::offer::Solution;

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
        // 注意如果删除第一个节点，那么新创建一个节点，把next指向head的下一个即可
        // 如果删除最后一个节点，那么只需要将最后一个节点的前一个节点的next指向None即可
        head
    }
}