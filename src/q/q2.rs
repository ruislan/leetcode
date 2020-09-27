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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 设置进位carry = 0
        // 同时移动l1和l2，将当前所指l1和l2以及carry加起来，如果超过十，carry = 1
        // 注意l1或者l2有可能先结束，那么后续只运算有数字的那个链表的值
        // Passed 0ms 2mb
        let mut head = Box::new(ListNode { val: 0, next: None });
        let (mut l1, mut l2, mut carry, mut cur) = (l1, l2, 0, head.as_mut());
        while l1.is_some() || l2.is_some() {
            let mut num = carry;
            if let Some(mut node) = l1 {
                num += node.val;
                l1 = node.next;
            }
            if let Some(mut node) = l2 {
                num += node.val;
                l2 = node.next;
            }
            carry = num / 10;
            cur.next = Some(Box::new(ListNode { val: num % 10, next: None }));
            cur = cur.next.as_mut().unwrap();
        }
        if carry > 0 { cur.next = Some(Box::new(ListNode { val: carry, next: None })); }
        head.next
    }
}

#[test]
fn test() {
    assert_eq!(Solution::add_two_numbers(None, None), None);
    assert_eq!(Solution::add_two_numbers(
        Some(Box::new(ListNode {
            val: 1,
            next:
            Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
        })),
        Some(Box::new(ListNode {
            val: 9,
            next:
            Some(Box::new(ListNode {
                val: 9,
                next:
                Some(Box::new(ListNode { val: 9, next: None })),
            })),
        })),
    ),
               Some(Box::new(ListNode {
                   val: 0,
                   next:
                   Some(Box::new(ListNode {
                       val: 1,
                       next:
                       Some(Box::new(ListNode {
                           val: 0,
                           next:
                           Some(Box::new(ListNode { val: 1, next: None })),
                       })),
                   })),
               }))
    );
    assert_eq!(Solution::add_two_numbers(
        Some(Box::new(ListNode {
            val: 2,
            next:
            Some(Box::new(ListNode {
                val: 4,
                next:
                Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 5,
            next:
            Some(Box::new(ListNode {
                val: 6,
                next:
                Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    ),
               Some(Box::new(ListNode {
                   val: 7,
                   next:
                   Some(Box::new(ListNode {
                       val: 0,
                       next:
                       Some(Box::new(ListNode { val: 8, next: None })),
                   })),
               }))
    );
}