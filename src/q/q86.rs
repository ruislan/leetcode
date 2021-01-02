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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 创建两个dummy_head,dh1和dh2
        // 遍历head，当val < x，连接到d1，否则连接到d2
        // 然后将d2接到d1的后面
        // 返回结果
        // Passed 0ms 2mb
        let mut head = head;
        let mut dh1 = Box::new(ListNode { val: 0, next: None });
        let mut dh1_mut = dh1.as_mut();
        let mut dh2 = Box::new(ListNode { val: 0, next: None });
        let mut dh2_mut = dh2.as_mut();

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                dh1_mut.next = Some(node);
                dh1_mut = dh1_mut.next.as_mut().unwrap();
            } else {
                dh2_mut.next = Some(node);
                dh2_mut = dh2_mut.next.as_mut().unwrap();
            }
        }

        dh1_mut.next = dh2.next;
        dh1.next
    }
}