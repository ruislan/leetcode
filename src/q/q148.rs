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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 利用q147的插入排序进行排序
        // 时间：O(n^2) 空间：O(1)
        // Passed 780ms 5.3mb
        // if head.is_none() { return head; }
        // let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        // let mut last = &mut dummy.as_mut().unwrap().next as *mut Option<Box<ListNode>>;
        // unsafe {
        //     while (*last).as_ref().unwrap().next.is_some() {
        //         if (*last).as_ref().unwrap().val <= (*last).as_ref().unwrap().next.as_ref().unwrap().val {
        //             last = &mut (*last).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
        //         } else {
        //             let mut unsorted = (*last).as_mut().unwrap().next.take();
        //             let mut ptr = &mut dummy as *mut Option<Box<ListNode>>;
        //             while (*ptr).as_ref().unwrap().next.is_some() &&
        //                 (*ptr).as_ref().unwrap().next.as_ref().unwrap().val <= unsorted.as_ref().unwrap().val {
        //                 ptr = &mut (*ptr).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
        //             }
        //             (*last).as_mut().unwrap().next = unsorted.as_mut().unwrap().next.take();
        //             unsorted.as_mut().unwrap().next = (*ptr).as_mut().unwrap().next.take();
        //             (*ptr).as_mut().unwrap().next = unsorted;
        //         }
        //     }
        // }
        // dummy.unwrap().next

        // 方法2
        // 将所有的节点全部断掉，然后放入数组vec
        // 数组进行快速排序
        // 然后从头开始串联链表
        // O(nlogn) 空间：O(n)
        // Passed 16ms 5.9mb
        // let mut nodes = Vec::new();
        // let mut head = head;
        // while let Some(mut node) = head {
        //     head = node.next.take();
        //     nodes.push(node);
        // }
        // nodes.sort_unstable_by(|a, b| b.val.cmp(&a.val));
        // let mut dummy = ListNode { val: 0, next: None };
        // let mut ptr = &mut dummy;
        // while let Some(node) = nodes.pop() {
        //     ptr.next = Some(node);
        //     ptr = ptr.next.as_mut().unwrap();
        // }
        // dummy.next

        // 方法3
        // 方法2魔改一下，不存储节点，直接存储值，与方法2没有本质上的区别
        // Passed 12ms 5.7mb
        let mut values = Vec::new();
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            values.push(node.val);
        }
        values.sort_unstable();
        let mut head = None;
        while let Some(val) = values.pop() {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head

        // 方法4
        // 归并排序，需要自底向上，满足O(nlogn), O(1)
        // TODO 没时间了，打个记号，后面来弄吧
    }
}