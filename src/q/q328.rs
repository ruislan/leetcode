use crate::q::Solution;

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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 用kotlin已经通过
        // 依靠三个指针p0 = head,p1 = head.next 和p_even = head.next
        // 只要while p0 != None && p0.next != None :
        //      p0.next = p1.next, p0 = p0.next
        //      p1.next = p0.next, p1 = p1.next
        // 最后p0.next = p_even，返回head即可，
        // 这里改成rust实现就行

        // 方法2
        // 我们创建一个计数器i，两个数据head_odd和head_even，
        // 以及各自的可变指针p_odd、p_even，这样5个变量满足空间O(1)的需求，
        // 然后迭代head，依次take掉head的当前节点，i为奇数给p_odd，i为偶数给p_even
        // 直到head没有了节点为止，最后将p_odd和head_even连接起来，返回head_odd即可
        // 这个方法我不知道是否满足原地，暂时也不想验证了，后面再说。
        // Passed 0ms 2.2mb
        let mut head = head;
        let (mut head_odd, mut head_even) = (Some(Box::new(ListNode { val: 0, next: None })), Some(Box::new(ListNode { val: 0, next: None })));
        let (mut p_odd, mut p_even) = (head_odd.as_mut(), head_even.as_mut());
        let mut i = 1;
        while let Some(mut node) = head {
            head = node.next.take();
            if i & 1 == 1 {
                if let Some(p) = p_odd {
                    p.next = Some(node);
                    p_odd = p.next.as_mut();
                }
            } else {
                if let Some(p) = p_even {
                    p.next = Some(node);
                    p_even = p.next.as_mut();
                }
            }
            i += 1;
        }
        if let Some(p) = p_odd { p.next = head_even.unwrap().next; }
        head_odd.unwrap().next
    }
}