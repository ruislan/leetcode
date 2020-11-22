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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 创建一个前直接点Node和指向前直接点的指针prev
        // 当prev.next不为None的时候，
        // 如果val与val相同，则将prev.next指向prev.next.next
        // 如果val与val不相同，prev指针移动到下一个节点
        // 返回前直接点的Node的下一个,node.next
        let mut node = Box::new(ListNode { val: 0, next: head });
        let mut prev = node.as_mut();
        while prev.next.is_some() {
            if prev.next.as_ref().unwrap().val == val {
                prev.next = prev.next.as_mut().unwrap().next.take();
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }
        node.next
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_elements(None, 0), None);
    assert_eq!(Solution::remove_elements(
        Some(Box::new(ListNode { val: 1, next: None })), 0),
               Some(Box::new(ListNode { val: 1, next: None })),
    );
    assert_eq!(Solution::remove_elements(
        Some(Box::new(ListNode { val: 1, next: None })), 1),
               None,
    );
    assert_eq!(Solution::remove_elements(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })), 1),
               Some(Box::new(ListNode { val: 2, next: None })),
    );
    assert_eq!(Solution::remove_elements(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })), 2),
               Some(Box::new(ListNode { val: 1, next: None })),
    );
    assert_eq!(Solution::remove_elements(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: None,
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })), 6),
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
               })),
    );
}