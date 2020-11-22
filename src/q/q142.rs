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
    // 本题没有Rust解入口，用Kotlin解决
    fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 利用sets存储node，出现第一个存在的node，即为有环
        // 完成迭代，返回空
        // kotlin code Passed 120ms 32mb
        //        val sets = HashSet<ListNode>()
        //        var node = head
        //        while (node != null) {
        //            if (!sets.add(node)) return node
        //            node = node.next
        //        }
        //        return null

        // 方法2
        // 不使用额外存储，用快慢指针，
        // 快指针走两个节点，满指针走一个节点
        // 当他们第一次相遇时，快指针可能走了n圈，慢指针最多一圈，
        // 假设距离分成三段，入圈前为a,圈中fast和slow相遇前的距离是b，相遇后到入圈的距离为c
        // 那么相遇时，快指针走了入圈前的距离和n圈的距离和追上slow的距离，fast = a + n(b + c) + b
        // 那么fast = a + (n + 1)b + nc
        // 那么slow = a + b
        // 又有Fast是slow的2倍，所以fast = 2*slow
        // a + (n + 1)b + nc = 2(a + b)
        // 2a - a = (n + 1)b + nc - 2b
        // a = (n - 1)b + nc
        // a = (n - 1)b + (n - 1)c + c
        // a = (n - 1)(b + c) + c
        // 而b + c正好等于一圈，一圈的距离我们可以认为没有移动，即可以消除
        // 所以得到a = c，即是说入圈前的距离a等于slow和fast相遇的距离到入圈的距离c
        // 所以程序可以先让两个指针相遇，然后再用一个指针ptr指向起点，然后同时移动ptr和slow，
        // 当他们相遇时，即为起点
        // kotlin code Passed 124ms 32.2mb
        // var slow = head
        // var fast = head
        // while (fast?.next != null) {
        //     slow = slow?.next
        //     fast = fast.next?.next
        //     if (slow == fast) {
        //         var ptr = head
        //         while (ptr != slow) {
        //             ptr = ptr?.next
        //             slow = slow?.next
        //         }
        //         return slow
        //     }
        // }
        // return null
        let (mut slow, mut fast) = (head.as_ref(), head.as_ref());
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
            if slow == fast {
                let mut ptr = head.as_ref();
                while ptr != slow {
                    ptr = ptr.unwrap().next.as_ref();
                    slow = slow.unwrap().next.as_ref();
                }
                return slow.cloned();
            }
        }
        None
    }
}