use crate::offer::Solution;

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 迭代链表，将所有的值取出来放入stack
        // 然后pop所有的值，用node包裹，next指向前一个节点

        // 方法2
        // 创建临时的指针prev表示上一个节点，初始为None
        // 逐步访问链表，将当前节点的next指向prev
        // 然后将当前节点作为前一个节点继续迭代
        head
    }
}