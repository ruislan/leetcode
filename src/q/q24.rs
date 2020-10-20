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

use crate::q::Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 要求需要实际的进行节点交换
        // 方法1
        // 将所有的节点都断开分别存储到数组中
        // 两两交换数组中的邻居
        // 迭代数组，把所有的节点都串起来
        // Passed 0ms 2.1mb
        // let mut nodes = Vec::new();
        // let mut head = head;
        // while let Some(mut node) = head {
        //     head = node.next.take();
        //     nodes.push(node);
        // }
        // let mut i = 1;
        // while i < nodes.len() {
        //     nodes.swap(i, i - 1);
        //     i += 2;
        // }
        // let mut head = Some(Box::new(ListNode { next: None, val: 0 }));
        // let mut ptr = head.as_mut();
        // for node in nodes {
        //     if let Some(p) = ptr {
        //         p.next = Some(node);
        //         ptr = p.next.as_mut();
        //     }
        // }
        // head.unwrap().next

        // 方法2
        // 优化，去掉数组，直接从节点中进行操作
        // 设置一个前置节点，然后将从前置节点开始，
        // 将前节点的下一个节点设置为当前节点
        // 将前节点的下下个节点设置为下个节点
        // 当当前节点和下个节点都存在时，取出下下个节点，
        // 然后将前个节点指向下个节点，将下个节点指向当前节点，最后将当前节点指向下下个节点
        // 然后将前置节点向后跳两个节点，如果不存在后两个节点，则直接退出
        // Passed 0ms 1.9mb
        let mut head = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut prev = head.as_mut();
        while let Some(node) = prev {
            if node.next.is_none() || node.next.as_ref().unwrap().next.is_none() { break; }
            let mut cur = node.next.take();
            let mut next = cur.as_mut().unwrap().next.take();
            let next_next = next.as_mut().unwrap().next.take();
            node.next = next;
            node.next.as_mut().unwrap().next = cur;
            node.next.as_mut().unwrap().next.as_mut().unwrap().next = next_next;
            prev = node.next.as_mut().unwrap().next.as_mut()
        }
        head.unwrap().next
    }
}

#[test]
fn test() {
    assert_eq!(Solution::swap_pairs(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            })),
        }))
    ),
               Some(Box::new(ListNode {
                   val: 2,
                   next: Some(Box::new(ListNode {
                       val: 1,
                       next: Some(Box::new(ListNode {
                           val: 3,
                           next: None,
                       })),
                   })),
               }))
    )
}