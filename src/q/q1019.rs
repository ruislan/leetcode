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
        ListNode { next: None, val }
    }
}
#[allow(unused)]
impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        // 方法1
        // 这个是伪装的单调栈
        // 单调栈就是遇到比栈内数据大的，就清空栈，并存入最大的那个
        // AC 24ms 2.7mb
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut answer = Vec::new();
        let mut head = head;
        let mut i = 0;
        while let Some(mut node) = head {
            answer.push(0);
            while !stack.is_empty() && stack.last().unwrap().1 < node.val {
                let last = stack.pop().unwrap();
                answer[last.0] = node.val;
            }
            stack.push((i, node.val));
            head = node.next.take();
            i += 1;
        }
        answer
    }
}
