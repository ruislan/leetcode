#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // 方法1
        // 最简单的方案就是将所有的ListNode都断开存储到vec中，
        // 然后通过比较数字大小得到一个升序的数组
        // 然后重新组合成为list即可
        // AC 0ms 3.1mb
        // let mut arr = Vec::new();
        // for mut list in lists {
        //     while list.is_some() {
        //         let node = list.as_mut().unwrap().next.take();
        //         arr.push(list);
        //         list = node;
        //     }
        // }
        // arr.sort_unstable_by(|a, b| a.as_ref().unwrap().val.cmp(&b.as_ref().unwrap().val));
        // let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        // let mut ptr = head.as_mut();
        // for x in arr {
        //     ptr.as_mut().unwrap().next = x;
        //     ptr = ptr.unwrap().next.as_mut();
        // }
        // head.unwrap().next

        // 方法2
        // 我们通过两两合并来操作，这样不需要排序
        // 所以我们需要一个帮助函数用来合并两个数组
        // 然后我们两两合并就行了
        // 就像归并排序一样
        // AC 144ms 3.1mb
        fn merge(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut dummy = Some(Box::new(ListNode { val:0, next:None }));
            let mut ptr = dummy.as_mut();
            while l1.is_some() || l2.is_some() {
                if l2.is_none() {
                    ptr.as_mut().unwrap().next = l1;
                    break;
                } 
                if l1.is_none() {
                    ptr.as_mut().unwrap().next = l2;
                    break;
                } 
                
                if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                    let l1_next = l1.as_mut().unwrap().next.take();
                    ptr.as_mut().unwrap().next = l1;
                    l1 = l1_next;
                } else {
                    let l2_next = l2.as_mut().unwrap().next.take();
                    ptr.as_mut().unwrap().next = l2;
                    l2 = l2_next;
                }
                ptr = ptr.unwrap().next.as_mut();
            }
            dummy.unwrap().next
        }

        let mut head = None;
        for mut list in lists { head = merge(head, list); }
        head
    }
}
