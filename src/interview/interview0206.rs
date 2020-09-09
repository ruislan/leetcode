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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // 方法1
        // 最简单方案
        // 遍历head，放入数组，然后判断数组是否是回文
        // Passed 4ms 4.1mb
        // let mut arr = Vec::new();
        // let mut head = head;
        // while let Some(node) = head.as_mut() {
        //     arr.push(node.val);
        //     head = node.next.take();
        // }
        // let (mut p1, mut p2) = (0, arr.len().saturating_sub(1));
        // while p1 < p2 {
        //     if arr[p1] != arr[p2] { return false; }
        //     p1 += 1;
        //     p2 -= 1;
        // }
        // true

        // 方法2
        // 达到O(1)的容积
        // 我们首先要获得链表的长度len，O(n)
        // 然后得到mid，和数量是否是奇偶odd
        // 再次迭代链表，到达mid，
        // 如果是奇数，放弃中点，如果是偶数，则直接将mid后的链表拿出来翻转
        // 然后同时迭代两个链表，比对值
        // Passed 4ms 3.9mb 并没少多少内存:(
        // 计算长度
        let mut len = 0;
        let mut node = head.as_ref();
        while let Some(cur) = node {
            len += 1;
            node = cur.next.as_ref();
        }

        // 0-1个都算回文
        if len < 2 { return true; }

        // 截取中部右边的链表
        let mut right = None;
        let mut head = head;
        let mid = (len >> 1) - 1;
        let mut num = 0;
        let mut node = head.as_mut();
        while let Some(cur) = node {
            if num == mid { right = cur.next.take(); }
            num += 1;
            node = cur.next.as_mut();
        }
        if len & 1 == 1 { right = right.unwrap().next; }

        // 翻转右边的链表
        let mut right = right;
        let mut node = None;
        while let Some(mut cur) = right {
            let mut next = cur.next.take();
            cur.next = node;
            node = Some(cur);
            right = next;
        }

        // 比较左右两个链表的值
        let mut left = head;
        let mut right = node;
        while left.is_some() && right.is_some() {
            if left.as_ref().unwrap().val != right.as_ref().unwrap().val { return false; }
            left = left.unwrap().next;
            right = right.unwrap().next;
        }

        left.is_none() && right.is_none()
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::is_palindrome(None), true);
    // let node = Some(Box::new(ListNode { val: 1, next: None }));
    // assert_eq!(Solution::is_palindrome(node), true);
    //
    // let node = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: None,
    //     })),
    // })
    // );
    // assert_eq!(Solution::is_palindrome(node), false);

    let node = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: None,
                })),
            })),
        })),
    })
    );
    assert_eq!(Solution::is_palindrome(node), true);
}