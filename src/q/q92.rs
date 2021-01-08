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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 只有一次遍历，那么我们只要进入了翻转区域[m..n]，就开始执行翻转
        // 然后翻转完成之后，把最后的接上去就可以了
        // Passed 0ms 1.9mb
        let mut dm = Some(Box::new(ListNode { val: 0, next: head }));
        let mut dm_mut = dm.as_mut();
        let mut rev = None;
        let mut cur = 0;
        while let Some(node) = dm_mut {
            cur += 1;
            while cur >= m && cur <= n {
                let mut next = node.next.take();
                node.next = next.as_mut().unwrap().next.take();
                next.as_mut().unwrap().next = rev.take();
                if cur < n {
                    rev = next;
                } else {
                    let mut rev_mut = next.as_mut();
                    while let Some(rev_node) = rev_mut {
                        if rev_node.next.is_none() {
                            rev_node.next = node.next.take();
                            break;
                        }
                        rev_mut = rev_node.next.as_mut();
                    }
                    node.next = next;
                    break;
                }
                cur += 1;
            }
            dm_mut = node.next.as_mut();
        }
        dm.unwrap().next
    }
}