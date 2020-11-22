use crate::interview::Solution;

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
    pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
        // 方法1
        // 直接遍历链表，然后取出len - k 个即可
        // let mut head = head.as_ref();
        // let mut values = Vec::new();
        // while let Some(node) = head {
        //     values.push(node.val);
        //     head = node.next.as_ref();
        // }
        // values[values.len() - k as usize]

        // 方法2
        // 双指针方法，p1先移动，当到达k长度的时候，
        // p2和p1同时移动，当p1到达尾部时，p2的值就是结果
        // 这个方法不需要额外的数组来存储数据
        let (mut p1, mut p2) = (head.as_ref(), head.as_ref());
        let mut gap = 0;
        while p1.is_some() {
            gap += 1;
            p1 = p1.unwrap().next.as_ref();
            if gap > k { p2 = p2.unwrap().next.as_ref(); }
        }
        match p2 {
            None => panic!(),
            Some(p2) => p2.val
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::kth_to_last(Some(Box::new(ListNode {
        val: 1,
        next: None,
    })), 1), 1);

    assert_eq!(Solution::kth_to_last(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }, )),
        1),
               3,
    );
}
