use crate::q::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

#[allow(unused)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 判断下一个节点与当前节点是否相等，
        // 如果相等，则cur.next = cur.next.next
        // 否则，cur = cur.next;
        // AC 0ms 2mb
        let mut head = head;
        let mut ptr = head.as_mut();
        while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
            let cur_val = ptr.as_ref().unwrap().val;
            let next_val = ptr.as_ref().unwrap().next.as_ref().unwrap().val;
            if cur_val == next_val {
                let mut next = ptr.as_mut().unwrap().next.as_mut().unwrap().next.take();
                ptr.as_mut().unwrap().next = next;
            } else {
                ptr = ptr.unwrap().next.as_mut();
            }
        }
        head
    }
}