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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 首先求出总长度n，然后看k与总长度的求余数
        // 如果余数是0，那就等于没转，直接返回head就行
        // 否则，将链表分成A,B两段，分段点就在n - k的位置
        // 然后将AB变成BA，返回即可
        let mut n = 0;
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            n += 1;
            ptr = node.next.as_ref();
        }
        let k = k % n;
        if k == 0 {
            return head;
        }
        let k = n - k;

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut i = 0;
        let mut ptr = dummy.as_mut();
        while i < k {
            ptr = ptr.unwrap().next.as_mut();
            i += 1;
        }
        let mut new_head = ptr.unwrap().next.take();
        let mut ptr = new_head.as_mut();
        while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
            ptr = ptr.unwrap().next.as_mut();
        }
        ptr.unwrap().next = dummy.unwrap().next.take();
        new_head
    }
}
