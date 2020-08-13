use crate::offer::Solution;

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 迭代链表，将所有的值取出来放入数组，顺序迭代数组，将值包装成节点，同时将该节点的下一个节点指向之前的那个节点
        // 0ms 2.5mb
        // let mut arr = Vec::new();
        // let mut node = head.as_ref();
        // while let Some(cur) = node {
        //     arr.push(cur.val);
        //     node = cur.next.as_ref();
        // }
        // let mut head = None;
        // arr.into_iter().for_each(|x| {
        //     head = Some(Box::new(ListNode { val: x, next: head.take() }));
        // });
        // head

        // 方法2
        // 方法1改良一下，不需要创建数组，直接在迭代时候创建出新的链表，更简洁，但是效率上还是创建了新的空间
        // 0ms 2.6mb
        // let mut head = head.as_ref();
        // let mut node = None;
        // while let Some(cur) = head {
        //     node = Some(Box::new(ListNode { val: cur.val, next: node.take() }));
        //     head = cur.next.as_ref();
        // }
        // node

        // 方法3
        // 改良方法2，在原链表上进行操作，不用新的
        // Passed 0ms 2.3mb
        let mut head = head;
        let mut node = None;
        while head.is_some() {
            let mut next = head.as_mut()?.next.take();
            head.as_mut()?.next = node;
            node = head;
            head = next;
        }
        node
    }
}

#[test]
fn test() {
    let a = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: None })) }));
    let b = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 1, next: None })) }));
    assert_eq!(Solution::reverse_list(a), b);
}