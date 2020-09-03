use crate::interview::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn delete_node(node: Option<Box<ListNode>>) {
        // 这道题没有Rust的版本，我们这里相当于用Rust作为伪代码写一下，
        // 然后用kotlin..，额，kotlin也不支持，那就用Java来作答吧，
        // 删除中间节点意味着当前节点的前后都应该有节点，否则输入不符合题目描述（中间节点）
        // 所以当前输入的节点最少前面有一个，后面有一个

        // 方法1
        // 假设输入1->2->3，那么输入的肯定是2，要变成1->3
        // 其实我们可以将3的值拷贝过来，然后再删除掉最后一个节点
        // 总结起来就是
        // 设置两个指针p1,p2，p1指向当前节点的下一个，p2指向当前节点，然后p1和p2同时移动
        // 将p1指向的值赋值给p2
        // 当p1移动到下一个是null时，将p2的next指向null
        // 不是null时,p2移动到下一个
        // let mut node = node;
        // let mut p1 = &node as *const Option<Box<ListNode>>;
        // let mut p2 = &mut node as *mut Option<Box<ListNode>>;
        // unsafe {
        //     p1 = &(*p1).as_ref().unwrap().next;
        //     while (*p1).is_some() {
        //         (*p2).as_mut().unwrap().val = (*p1).as_ref().unwrap().val;
        //         p1 = &(*p1).as_ref().unwrap().next;
        //         if (*p1).is_none() {
        //             (*p2).as_mut().unwrap().next = None;
        //         } else {
        //             p2 = &mut (*p2).as_mut().unwrap().next;
        //         }
        //     }
        // }
        // Java 版本Passed 0ms 39mb
        // public void deleteNode(ListNode node) {
        //     ListNode p1 = node.next;
        //     ListNode p2 = node;
        //     while (p1 != null) {
        //         p2.val = p1.val;
        //         p1 = p1.next;
        //         if (p1 == null) p2.next = null;
        //         else p2 = p2.next;
        //     }
        // }

        // 方法2
        // 优化方法1，我们其实不用依次赋值
        // 实际上我们只需要将下一个节点的值赋值给当前节点，
        // 然后把下一个节点给删除掉（即是将当前节点的下一个节点指向下下个节点）
        let mut node = node;
        node.as_mut().unwrap().val = node.as_mut().unwrap().next.as_mut().unwrap().val;
        node.as_mut().unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();
        // Java 版本 0ms 39mb
        // node.val = node.next.val;
        // node.next = node.next.next;
    }
}