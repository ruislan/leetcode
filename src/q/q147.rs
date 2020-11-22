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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 首先注入一个前置的节点dummy
        // 链表由三部分组成，有序 -> 待处理节点 -> 无序
        // 我们要做的事情就是，将待处理节点插入到有序中，然后从无序中取出第一个作为待处理节点，继续重复，直到无序没有了
        // 因为无法从后向前遍历，所以每次插入都要从头遍历有序，这其实是将插入排序在大部分有序的情况下很快的优点给抹掉了
        // 最后返回dummy.next
        // Passed 28ms 2.3mb O(n^2)
        // let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        // let mut unsorted = head;
        // while unsorted.is_some() {
        //     let tail = unsorted.as_mut().unwrap().next.take();
        //     let mut ptr = dummy.as_mut();
        //     while ptr.is_some() {
        //         if ptr.as_ref().unwrap().next.is_none() ||
        //             ptr.as_ref().unwrap().next.as_ref().unwrap().val > unsorted.as_ref().unwrap().val {
        //             unsorted.as_mut().unwrap().next = ptr.as_mut().unwrap().next.take();
        //             ptr.as_mut().unwrap().next = unsorted;
        //             break;
        //         }
        //         ptr = ptr.unwrap().next.as_mut();
        //     }
        //     unsorted = tail;
        // }
        // dummy.unwrap().next

        // 方法2
        // 方法1其实不能算是原地插入，实际上是将节点一个一个拿出来，然后插入到dummy中
        // 而这题的原地的意思是即便dummy，也需要连接上链表，然后再其上操作
        // 所以我们将方法1进行修改
        // 思路还是不变，有序 -> 待处理节点 -> 无序
        // 关键点就是我们要知道有序的最右边是哪个，并且保持其不动
        // 由于我们要使用两个可变指针，所以我们用raw指针搞
        // Passed 4ms 2.4mb O(n^2)
        if head.is_none() { return head; }
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut last = &mut dummy.as_mut().unwrap().next as *mut Option<Box<ListNode>>;
        unsafe {
            while (*last).as_ref().unwrap().next.is_some() {
                if (*last).as_ref().unwrap().val <= (*last).as_ref().unwrap().next.as_ref().unwrap().val {
                    last = &mut (*last).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                } else {
                    // dummy(sorted) -> ... last -> None | unsorted-> tail->... -> None
                    let mut unsorted = (*last).as_mut().unwrap().next.take();
                    let mut ptr = &mut dummy as *mut Option<Box<ListNode>>;
                    while (*ptr).as_ref().unwrap().next.is_some() && (*ptr).as_ref().unwrap().next.as_ref().unwrap().val <= unsorted.as_ref().unwrap().val {
                        ptr = &mut (*ptr).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                    }
                    (*last).as_mut().unwrap().next = unsorted.as_mut().unwrap().next.take();
                    unsorted.as_mut().unwrap().next = (*ptr).as_mut().unwrap().next.take();
                    (*ptr).as_mut().unwrap().next = unsorted;
                    // dummy(sorted) -> ... -> unsorted -> ...-> last -> tail->... -> None
                }
            }
        }
        dummy.unwrap().next
    }
}

#[test]
fn test() {
    assert_eq!(Solution::insertion_sort_list(
        Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                })),
            })),
        }))),
               Some(Box::new(ListNode {
                   val: 1,
                   next: Some(Box::new(ListNode {
                       val: 2,
                       next: Some(Box::new(ListNode {
                           val: 3,
                           next: Some(Box::new(ListNode {
                               val: 4,
                               next: None,
                           })),
                       })),
                   })),
               }))
    );
}