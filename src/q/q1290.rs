use crate::q::Solution;

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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        // 方法一：用vec重新存储所有的数字，然后用堆栈先进后出来进行10进制计算
        // Passed 0ms 2mb
        // let mut head = *head.unwrap();
        // let mut nums = Vec::new();
        // loop {
        //     nums.push(head.val);
        //     if head.next == None { break; }
        //     head = *head.next.unwrap();
        // }
        // let mut p = 0;
        // let mut res = 0;
        // while let Some(x) = nums.pop() {
        //     res += 2_i32.pow(p) * x;
        //     p += 1;
        // }
        // res

        // 方法二：用二进制进行左移动操作，出来一个数字，所有左移一位加上当前的数字
        // Passed 0ms 2mb
        // let mut res = 0;
        // let mut head = *head.unwrap();
        // loop {
        //     res <<= 1;
        //     res += head.val;
        //     if head.next == None { break; }
        //     head = *head.next.unwrap();
        // }
        // res

        // 方法二更graceful的写法
        // Passed 0ms 2mb
        let mut res = 0;
        let mut head = &head;
        while let Some(node) = head {
            res = res << 1 | node.val;
            head = &node.next;
        }
        res
    }
}