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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 创建一个数组存储两个链表的值，并排序，然后再组合成链表
        // Passed 4ms 2.1mb
        // let mut arr = Vec::new();
        // let mut l1 = l1.as_ref();
        // while let Some(cur) = l1 {
        //     arr.push(cur.val);
        //     l1 = cur.next.as_ref();
        // }
        // let mut l2 = l2.as_ref();
        // while let Some(cur) = l2 {
        //     arr.push(cur.val);
        //     l2 = cur.next.as_ref();
        // }
        // arr.sort_unstable();
        // let mut node = None;
        // while let Some(val) = arr.pop() {
        //     node = Some(Box::new(ListNode { val, next: node }));
        // }
        // node

        // 方法2
        // 我们排序的话就是O(nlogn)，其实没必要排序，我们用两个指针分别指向两个链表，p1,p2
        // 当 p1 != None || p2 != None
        // 如果p2 == None || p1.val <= p2.val，take p1.val，且让p1 = p1.next
        // 如果p1 == None || p1.val > p2.val，take p2.val，且让p2 = p2.next
        // Passed 4ms 2.1mb
        // let (mut p1, mut p2) = (l1.as_ref(), l2.as_ref());
        // let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        // let mut node = head.as_mut();
        //
        // while p1.is_some() && p2.is_some() {
        //     let mut val = 0;
        //     if p1.unwrap().val <= p2.unwrap().val {
        //         val = p1.unwrap().val;
        //         p1 = p1.unwrap().next.as_ref();
        //     } else {
        //         val = p2.unwrap().val;
        //         p2 = p2.unwrap().next.as_ref();
        //     }
        //     node.as_mut().unwrap().next = Some(Box::new(ListNode { val, next: None }));
        //     node = node.unwrap().next.as_mut();
        // }
        //
        // while p1.is_some() {
        //     node.as_mut().unwrap().next = Some(Box::new(ListNode { val: p1.unwrap().val, next: None }));
        //     node = node.unwrap().next.as_mut();
        //     p1 = p1.unwrap().next.as_ref();
        // }
        //
        // while p2.is_some() {
        //     node.as_mut().unwrap().next = Some(Box::new(ListNode { val: p2.unwrap().val, next: None }));
        //     node = node.unwrap().next.as_mut();
        //     p2 = p2.unwrap().next.as_ref();
        // }
        //
        // head.unwrap().next

        // 方法3
        // 是方法2的改良，我们不需要新创建节点，我们只需要把现有节点断掉，拼接就可以了，这样不用用新的空间
        // Passed 0ms 2.1mb
        let (mut l1, mut l2) = (l1, l2);
        let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut node = head.as_mut();
        while l1.is_some() && l2.is_some() {
            let cur;
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let next = l1.as_mut().unwrap().next.take();
                cur = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                cur = l2;
                l2 = next;
            }
            node.as_mut().unwrap().next = cur;
            node = node.unwrap().next.as_mut();
        }
        node.as_mut().unwrap().next = if l1.is_some() { l1 } else { l2 };
        head.unwrap().next
    }
}