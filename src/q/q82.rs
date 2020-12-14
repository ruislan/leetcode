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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 要注意的是链表中的值不一定是有序的，
        // 所以用hashmap存储key:数字，value：节点序列（vec）
        // 迭代map，如果value.len() > 1，从map中删除
        // 迭代head，每个节点的值如果不在map中就删除
        None
    }
}