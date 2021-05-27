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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 直接把数字拿出来，用数组做就简单多了吧
        // AC 4ms 2.3mb
        if k == 1 { return head; }
        let mut head = head;
        let mut values = Vec::new();
        while let Some(node) = head {
            values.push(node.val);
            head = node.next;
        }
        let k = k as usize;
        let n = values.len();
        for i in (0..n).step_by(k) {
            if i + k > n { break; }
            values[i..i + k].reverse();
        }
        let mut dummy = ListNode::new(0);
        let mut ptr = &mut dummy;
        for x in values {
            ptr.next = Some(Box::new(ListNode::new(x)));
            ptr = ptr.next.as_mut().unwrap();
        }
        dummy.next

        // 方法2
        // 其实Rust可以直接take数据，所以我们可以直接k个数目take一次
        // 然后将i..i+k个翻转，然后再接上，然后从i+k个继续走k个，直到结束
        // 这个方法后面来做吧，今天太晚了，睡了
    }
}