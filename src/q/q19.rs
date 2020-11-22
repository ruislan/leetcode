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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 将第n-1个节点的next节点指向这个next的next即可
        // Passed 0ms 1.9mb
        // let mut head = head;
        // let (mut cur, mut count) = (head.as_mut(), 0);
        // while let Some(node) = cur {
        //     count += 1;
        //     cur = node.next.as_mut();
        // }
        // count -= n;
        // let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        // cur = head.as_mut();
        // while let Some(node) = cur {
        //     if count == 0 {
        //         if let Some(mut next) = node.next.take() {
        //             node.next = next.next.take();
        //         }
        //         break;
        //     }
        //     count -= 1;
        //     cur = node.next.as_mut();
        // }
        // head.unwrap().next

        // 方法2
        // 一次循环，两个指针，p1, p2
        // p1先走，n步后，两个同时移动，
        // p1到达尾部时，p2就是那个要删除的节点，p2指向 p2.next即可
        // 由于Rust只能在一个时间拥有一个可变指针，或者多个不可变指针，而本题需要两个可变指针，所有使用unsafe方式
        // Passed 0ms 1.9mb
        let (mut head, mut n) = (head, n as u32);
        let mut p1 = &mut head as *mut Option<Box<ListNode>>;
        let mut p2 = &mut head as *mut Option<Box<ListNode>>;
        unsafe {
            while (*p1).is_some() {
                p1 = &mut (*p1).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                if n == 0 { p2 = &mut (*p2).as_mut().unwrap().next as *mut Option<Box<ListNode>>; }
                n = n.saturating_sub(1);
            }
            if (*p2).is_some() { p2.replace((*p2).as_mut().unwrap().next.take()); }
        }
        head
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_nth_from_end(None, 0), None);
    assert_eq!(Solution::remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 0), Some(Box::new(ListNode { val: 1, next: None })));
    assert_eq!(Solution::remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 1), None);
    assert_eq!(Solution::remove_nth_from_end(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None,
                        })),
                    })),
                })),
            })),
        }))
        , 2,
    ),
               Some(Box::new(ListNode {
                   val: 1,
                   next: Some(Box::new(ListNode {
                       val: 2,
                       next: Some(Box::new(ListNode {
                           val: 3,
                           next: Some(Box::new(ListNode {
                               val: 5,
                               next: None,
                           })),
                       })),
                   })),
               }))
    )
}