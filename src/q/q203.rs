use crate::q::Solution;

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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 创建一个前直接点Node和指向前直接点的指针prev
        // 当prev.next不为None的时候，
        // 如果val与val相同，则将prev.next指向prev.next.next
        // 如果val与val不相同，prev指针移动到下一个节点
        // 返回前直接点的Node的下一个,node.next
        None
    }
}