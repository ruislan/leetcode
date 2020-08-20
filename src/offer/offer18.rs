use crate::offer::Solution;

// Definition for singly-linked list.
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
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 删除某一个节点需要线性查找节点的值与val相同的节点，当找到这个节点之后
        // 将这个节点的prev节点的next指向这个节点的next
        // 注意如果删除第一个节点，那么将第一个节点的下一个节点作为头结点即可
        // 如果删除最后一个节点，那么只需要将最后一个节点的前一个节点的next指向None即可
        // Passed 0ms 2.1mb
        // let mut head = &head;
        // let mut vals = vec![];
        // while let Some(node) = head {
        //     if node.val != val { vals.push(node.val); }
        //     head = &node.next;
        // }
        // let mut head = None;
        // while let Some(val) = vals.pop() {
        //     head = Some(Box::new(ListNode { val, next: head }));
        // }
        // head

        // 方法2
        // 创建一个前置节点head的next指向当前节点head
        // 然后创建一个可变指针进行迭代，如果当前值相等，就拿走后面的链表
        // Passed 0ms 2.1mb
        // let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        // let mut cur = &mut head;
        // while let Some(node) = cur {
        //     let next = &mut node.next;
        //     match next {
        //         None => break,
        //         Some(next) => {
        //             if next.val == val {
        //                 node.next = next.next.take();
        //                 break;
        //             }
        //         }
        //     }
        //     cur = &mut node.next;
        // }
        // head.unwrap().next

        // 方法3
        // 创建一个前置节点head的next指向当前节点head
        // 直接判断当前节点，如果是第一个，返回后面的，如果不是，则把当前节点定为prev，然后检查prev.next
        // Passed 0ms 2.1mb
        if head.is_none() { return head; }
        if head.as_ref().unwrap().val == val { return head.unwrap().next; }
        let mut head = head;
        let mut node = head.as_mut();
        while let Some(prev) = node {
            match prev.next.as_mut() {
                None => break,
                Some(cur) => {
                    if cur.val == val {
                        prev.next = cur.next.take();
                        break;
                    }
                }
            }
            node = prev.next.as_mut();
        }
        head
    }
}