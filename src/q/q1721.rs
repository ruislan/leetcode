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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 在Rust中一个数据只能有一个可变指针，所以一些方法就被限制了，例如快慢指针
        // 当然，我们也可以使用unsafe的方式
        // AC 120ms 9.2mb
        let mut n = 0;
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            n += 1;
            ptr = node.next.as_ref();
        }

        let swap1 = k.min(n + 1 - k);
        let swap2 = k.max(n + 1 - k);
        if swap1 == swap2 { return head; }

        let mut head = head;
        let mut ptr = head.as_mut();
        let mut swap1_val = 0;
        let mut swap2_val = 0;
        let mut i = 1;
        while let Some(node) = ptr {
            if i == swap1 { swap1_val = node.val; }
            if i == swap2 { swap2_val = node.val; }
            i += 1;
            ptr = node.next.as_mut();
        }
        let mut ptr = head.as_mut();
        let mut i = 1;
        while let Some(node) = ptr {
            if i == swap1 { node.val = swap2_val; }
            if i == swap2 { node.val = swap1_val; }
            i += 1;
            ptr = node.next.as_mut();            
        }

        head
    }
}