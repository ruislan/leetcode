use std::hash::Hash;
use std::ops::Deref;

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
    // 本题没有Rust解入口，用Kotlin解决
    fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        // 方法1
        // 用hashset存储每个Node的内存地址，如果某个Node已经出现过了
        // 则说明存在环
        // Kotlin code Passed 200ms 34.4mb
        //
        //        var head = head
        //         val hashset = HashSet<ListNode>()
        //         while (head != null) {
        //             if (hashset.contains(head)) return true
        //             hashset.add(head)
        //             head = head.next
        //         }
        //         return false

        // Rust应该有更好的获取内存地址的方式，后面来改进吧
        // let mut head = head.as_ref();
        // let mut sets = std::collections::HashSet::new();
        // while let Some(node) = head {
        //     let addr = format!("{:p}", node);
        //     if sets.contains(&addr) { return true; }
        //     sets.insert(addr);
        //     head = node.next.as_ref()
        // }
        // false

        // 方法2
        // 快慢指针，假设是环形，那么快指针总会与慢指针回合
        // 如果不是环形，那么快指针会先跑到终点结束循环
        // kotlin code Passed 200ms 34.9mb
        //        var fast = head?.next
        //        var slow = head
        //        while (slow != null && fast != null) {
        //            if (fast == slow) return true
        //            fast = fast.next?.next
        //            slow = slow.next
        //        }
        //        return false
        let mut slow = head.as_ref();
        let mut fast = if let Some(node) = head.as_ref() { node.next.as_ref() } else { None };
        while slow.is_some() && fast.is_some() {
            if fast == slow { return true; }
            fast = if let Some(next) = fast.unwrap().next.as_ref() { next.next.as_ref() } else { None };
            slow = slow.unwrap().next.as_ref();
        }
        false
    }
}