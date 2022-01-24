use std::ops::Range;

use rand::prelude::*;

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

// 方法1
// 根据数组的特性，随机访问是O(1)的时间，所以可以使用此数据结构来缓存节点数据
// AC 8ms 4mbÏ
struct Solution {
    cache: Vec<i32>,
}

#[allow(unused)]
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut node = head.as_ref();
        let mut cache = Vec::new();
        while node.is_some() {
            let ptr = node.unwrap();
            cache.push(ptr.val);
            node = ptr.next.as_ref();
        }
        Solution { cache }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.cache[rng.gen_range(Range { start: 0, end: self.cache.len() })]
    }
}