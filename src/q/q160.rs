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
    // 此题没有Rust的入口，所以用kotlin回答
    // Kotlin Passed 172ms 36mb
    fn get_intersection_node(head_a: Option<Box<ListNode>>, head_b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 要求O(n)时间，O(1)空间
        // 方法1
        // 遍历a和b，得到长度，然后长的先出发，短的等到长的与自己相等后再出发
        // 如果两个节点在某处相等，则说明存在交点
        // 直到迭代结束，返回空
        let (mut len_a, mut len_b, mut p_a, mut p_b) = (0, 0, head_a.as_ref(), head_b.as_ref());
        while let Some(node) = p_a {
            len_a += 1;
            p_a = node.next.as_ref();
        }
        while let Some(node) = p_b {
            len_b += 1;
            p_b = node.next.as_ref();
        }
        let (mut long, short, mut p_long, mut p_short) = match len_a.cmp(&len_b) {
            std::cmp::Ordering::Greater => (len_a, len_b, head_a.as_ref(), head_b.as_ref()),
            _ => (len_b, len_a, head_b.as_ref(), head_a.as_ref()),
        };

        while p_long.is_some() && p_short.is_some() {
            let node_long = p_long.unwrap();
            let node_short = p_short.unwrap();
            if node_long == node_short { return Some(node_long.clone()); }
            if long > short {
                p_long = node_long.next.as_ref();
                long -= 1;
            } else {
                p_short = node_short.next.as_ref();
            }
        }

        None
    }
}