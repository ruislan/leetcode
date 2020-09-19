use crate::q::Solution;

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
    // 本题没有Rust提交窗口，用Kotlin完成
    pub fn delete_node(node: Option<&mut Box<ListNode>>) {
        // 方法1
        // 拷贝下一个节点的值到当前节点，然后断掉下一个节点，接上下下个节点
        if let Some(node) = node {
            node.val = node.next.as_ref().unwrap().val;
            node.next = node.next.as_mut().unwrap().next.take();
        }

        // Kotlin Passed 192ms 33.1mb
        // fun deleteNode(node: ListNode?) {
        //     node!!.`val` = node!!.next!!.`val`
        //     node!!.next = node!!.next!!.next
        // }
    }
}