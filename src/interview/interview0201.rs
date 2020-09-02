use crate::interview::Solution;

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
    pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 创建一个hashset，遍历链表，如果没出现这个值，就放入hashset
        // 8ms 3.3mb
        // 如果出现了这个值，就断掉这个子节点
        // let mut sets = std::collections::HashSet::new();
        // let mut head = Box::new(ListNode { val: -1, next: head });
        // let mut cur = head.as_mut();
        // while let Some(next) = cur.next.as_mut() {
        //     if !sets.insert(next.val) {
        //         cur.next = next.next.take();
        //     } else {
        //         cur = cur.next.as_mut().unwrap();
        //     }
        // }
        // head.next

        // 方法2
        // 方法1的改进，试试数组会不会快一点
        // Passed 4ms 3.2mb
        head.map(|mut head| {
            let mut cur = head.as_mut();
            let mut bucket = vec![0; 20001];
            bucket[cur.val as usize] = 1;
            while let Some(next) = cur.next.as_mut() {
                if bucket[next.val as usize] != 0 {
                    cur.next = next.next.take();
                } else {
                    bucket[next.val as usize] = 1;
                    cur = cur.next.as_mut().unwrap();
                }
            }
            head
        })
    }
}