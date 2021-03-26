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

use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 设置一个dummy，设置ptr1从dummy开始，设置i计数，
        // 当到a-1的位置，取下ptr1.next，这个就是要删除的部分的头部，
        // 然后设置ptr2从这个位置继续迭代，
        // 到b - 1的位置，取下ptr2.next，这个就是尾部要保留的tail
        // 然后ptr1.next = list2, 然后将ptr1迭代到ptr1.next = null的位置
        // 将ptr1.next = tail，最后返回dummy.next即可
        // AC 56ms 3.4mb
        let mut i = 0;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: list1 }));
        let mut ptr1 = dummy.as_mut();
        while i < a {
            i += 1;
            ptr1 = ptr1.unwrap().next.as_mut();
        }
        let mut remove_head = ptr1.as_mut().unwrap().next.take();
        ptr1.as_mut().unwrap().next = list2;
        let mut ptr2 = remove_head.as_mut();
        while i < b {
            i += 1;
            ptr2 = ptr2.unwrap().next.as_mut();
        }
        let mut tail = ptr2.unwrap().next.take();
        
        while ptr1.is_some() && ptr1.as_ref().unwrap().next.is_some() {
            ptr1 = ptr1.unwrap().next.as_mut();
        }
        ptr1.unwrap().next = tail;
        dummy.unwrap().next
    }
}