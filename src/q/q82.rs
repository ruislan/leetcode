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
        // 设置一个ptr为下一个节点，如果ptr与ptr下一个节点相同，那么ptr移动
        // 一直到ptr不与最初的节点值相同，
        // 那么如果有相同的节点出现就node.next = ptr.next
        // 否则就node = node.next继续判断
        // 这样一个遍历就能完成
        // kotlin代码
        // AC 228ms 34.8mb
        // var dm:ListNode? = ListNode(0)
        // dm?.next = head
        // var node = dm
        // while (node?.next != null) {
        //     var ptr = node?.next
        //     var dup = false
        //     while (ptr?.next != null && ptr?.next?.`val` == node?.next?.`val`) {
        //         ptr = ptr?.next
        //         dup = true
        //     }
        //     if (dup) node?.next = ptr?.next
        //     else node = node?.next
        // }
        // return dm?.next

        // 方法2
        // 两次遍历，统计值出现的频率，如果节点值的频率超过1，则不要该节点
        // AC 0ms 2mb
        let mut freq = std::collections::HashMap::new();
        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            *freq.entry(node.val).or_insert(0) += 1;
            ptr = node.next.as_ref();
        }

        let mut dummy = ListNode { val: 0, next: None };
        let mut ptr = &mut dummy;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if freq.get(&node.val) == Some(&1) {
                ptr.next = Some(node);
                ptr = ptr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}