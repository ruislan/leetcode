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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 先迭代完链表，求出总数，然后求得中间数mid
        // 然后到mid之后取当前位置的返回即可
        // Passed 0ms 1.9mb
        // let (mut ptr, mut len) = (head.as_ref(), 0);
        // while let Some(node) = ptr {
        //     len += 1;
        //     ptr = node.next.as_ref();
        // }
        //
        // let (mid, mut idx, mut head) = (len / 2, 0, head);
        // while let Some(mut node) = head {
        //     if idx == mid { return Some(node); }
        //     idx += 1;
        //     head = node.next;
        // }
        //
        // None

        // 方法2
        // 双指针，p0, p1，p0一次移动一步，p1一次移动两步，这样p1到达终点时，p0就在中间
        // 因为p0不可变指针，p1为可变指针，Rust有可变指针的情况下，其他指针都不可存在
        // 所以这里用Raw Pointer
        // Passed 0ms 2mb
        let mut head = head;
        let mut p0 = &mut head as *mut Option<Box<ListNode>>;
        let mut p1 = head.as_ref();
        unsafe {
            while p1.is_some() && p1.as_ref().unwrap().next.is_some() {
                p1 = p1.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();
                p0 = &mut (*p0).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            }
            (*p0).take()
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::middle_node(None), None);
    assert_eq!(
        Solution::middle_node(Some(Box::new(ListNode { val: 2, next: None }))),
        Some(Box::new(ListNode { val: 2, next: None }))
    );
    assert_eq!(
        Solution::middle_node(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None,
            })),
        }))),
        Solution::middle_node(Some(Box::new(ListNode {
            val: 2,
            next: None,
        })))
    );
    assert_eq!(
        Solution::middle_node(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            })),
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        }))
    );
}