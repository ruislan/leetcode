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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 迭代法
        // 两个指针p1和p2，p1指向None，p2指向head，
        // 由p2遍历head，如果p2指向的node不为空
        // 让p2指向node.next，让node.next指向p1，让p1指向node
        // 迭代结束，返回p1
        // Passed 0ms 2.3mb
        // let mut head = head;
        // let mut new_head = None;
        // while let Some(mut node) = head {
        //     head = node.next.take();
        //     node.next = new_head;
        //     new_head = Some(node);
        // }
        // new_head

        // 方法2
        // 递归方法
        // 递归就是找基线和缩小范围了
        // 我们看一下只有一个节点的时候，是不需要翻转的
        // 有两个节点的时候，把后一个节点的下一个位置指向前一个节点，前一个节点的下一个位置指向空
        // 那么三个节点可以看做是已经翻转的两个节点作为前一个节点和后一个节点，就又变成两个节点了
        // 又可以使用上面的的方式，直到所有的节点都处理完
        fn do_reverse(new_head: Option<Box<ListNode>>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                Some(mut node) => {
                    let head = node.next.take();
                    node.next = new_head;
                    do_reverse(Some(node), head)
                }
                None => new_head,
            }
        }
        do_reverse(None, head)
    }
}