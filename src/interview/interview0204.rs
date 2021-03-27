use crate::interview::Solution;

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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 创建dummy头节点是解决链表问题的总是会考虑的一个点
        // 将小的分成一组，大的分成一组即可
        // AC 0ms 1.9mb
        let mut le = ListNode { val: 0, next: None };
        let mut gt = ListNode { val: 0, next: None };

        let mut le_ptr = &mut le;
        let mut gt_ptr = &mut gt;
        let mut head = head;

        while let Some(mut node) = head {
            let mut next = node.next.take();
            if node.val < x {
                le_ptr.next = Some(node);
                le_ptr = le_ptr.next.as_mut().unwrap().as_mut();
            } else {
                gt_ptr.next = Some(node);
                gt_ptr = gt_ptr.next.as_mut().unwrap().as_mut();
            }
            head = next;
        }
        le_ptr.next = gt.next.take();
        le.next
    }
}







