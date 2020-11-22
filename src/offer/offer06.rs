use crate::offer::Solution;

// Definition for singly-linked list.
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
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        // 方法1
        // 遍历head，放入数组arr，然后将数组reverse即可
        // Passed 0ms 2.6mb
        // let mut head_ref = head.as_ref();
        // let mut arr = Vec::new();
        // while let Some(cur_ref) = head_ref {
        //     arr.push(cur_ref.val);
        //     head_ref = cur_ref.next.as_ref();
        // }
        // arr.into_iter().rev().collect()

        // 方法2
        // 方法1的优化版本，我们直接每次将head的所有权都移交，而不是用引用试试
        // Passed 0ms 2.5mb
        // 快得不明显
        let mut head = head;
        let mut arr = Vec::new();
        while let Some(cur) = head {
            arr.push(cur.val);
            head = cur.next;
        }
        arr.into_iter().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_print(None), vec![]);
    assert_eq!(Solution::reverse_print(Some(Box::new(ListNode { val: 2, next: None }))), vec![2]);
    assert_eq!(Solution::reverse_print(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None,
            })),
        })),
    }))), vec![2, 3, 1]);
}