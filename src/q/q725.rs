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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        // 方法1
        // 将所有的Node都断开，根据Node总数n来判断
        // 如果n > k，则直接用 each = n/k 来划分，同时如果不能整除，就通过余数m，在前m个部分多一个Node
        // 如果n <= k，则直接每个节点单独存储
        // AC 0ms 2mb 43/43
        let mut head = head;
        let mut node_list = Vec::new();
        while let Some(mut node) = head {
            head = node.next.take();
            node_list.push(Some(node));
        }
        let n = node_list.len();
        let k = k as usize;
        if n <= k {
            while node_list.len() != k { node_list.push(None); }
            return node_list;
        }

        let part = n / k;
        let mut rem = n % k;
        let mut ans = Vec::new();
        let mut node_idx = 0;
        while node_idx < n {
            let cnt = if rem > 0 {
                rem -= 1;
                part + 1
            } else { part };
            let limit = cnt + node_idx;

            let mut head = Some(Box::new(ListNode { val: 0, next: None }));
            let mut head_ptr = head.as_mut();
            while head_ptr.is_some() && node_idx < limit {
                head_ptr.as_mut().unwrap().next = node_list[node_idx].take();
                head_ptr = head_ptr.unwrap().next.as_mut();
                node_idx += 1;
            }
            ans.push(head.unwrap().next.take());
        }
        ans
    }
}