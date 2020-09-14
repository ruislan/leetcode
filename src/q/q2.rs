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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 设置进位carry = 0
        // 同时移动l1和l2，将当前所指l1和l2以及carry加起来，如果超过十，carry = 1
        // 注意l1或者l2有可能先结束，那么后续只运算有数字的那个链表的值
        None
    }
}