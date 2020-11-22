use crate::offer::Solution;

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
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 方法1
        // 线性探索链表，将值存入stack，然后pop k个node，
        // 将用node包裹当前值，且将next指向前一个pop的节点
        // Passed 0ms 2.1mb
        // let mut arr = Vec::new();
        // let mut head = head.as_ref();
        // while let Some(cur) = head {
        //     arr.push(cur.val);
        //     head = cur.next.as_ref();
        // }
        // let mut head = None;
        // for i in ((arr.len() - k as usize)..arr.len()).rev() {
        //     head = Some(Box::new(ListNode { next: head, val: arr[i] }));
        // }
        // head

        // 方法2
        // 方法1虽然能解决问题，但是不够优雅，没有利用到单链表的特性，而且还额外用了存储空间
        // 我们设置两个指针p1和p2，p1先出发，等p1和p2的位置与k相等时，p1和p2同时移动
        // 直到p1到达最后一个，那么p2即是我们需要的剩余的链表
        // Passed 0ms 2mb
        // let mut p1 = (head.as_ref(), 0);
        // let mut p2 = (head.as_ref(), 0);
        // while p1.0.is_some() {
        //     if p1.1 - p2.1 == k {
        //         p2 = (p2.0?.next.as_ref(), p2.1 + 1);
        //     }
        //     p1 = (p1.0?.next.as_ref(), p1.1 + 1);
        // }
        // p2.0.cloned()

        // 方法3
        // 方法2还是拷贝了一些数组，用了不少空间，所以我们改进方法1，遍历之后只记录数字，这样第二次就可以逐步断掉链接
        // 这个方法就是O(n)，内存就是O(1)
        // Passed 0ms 1.9mb
        // let mut size = 0;
        // let mut ptr = head.as_ref();
        // while let Some(cur) = ptr {
        //     size += 1;
        //     ptr = cur.next.as_ref();
        // }
        // let mut head = head;
        // for i in 0..(size - k as usize) {
        //     head = head.take()?.next;
        // }
        // head

        // 方法4
        // 采用裸指针来解决问题，这样我们就不需要额外的辅助了
        // Passed 0ms 2mb
        // 哎，居然没有上面那个运行得快也，这个是最优雅的了吧
        let (mut head, mut gap) = (head, 0);
        let mut p1 = &head as *const Option<Box<ListNode>>;
        let mut p2 = &mut head as *mut Option<Box<ListNode>>;
        unsafe {
            while (*p1).is_some() {
                if gap >= k { p2 = &mut (*p2).as_mut()?.next; }
                p1 = &(*p1).as_ref()?.next;
                gap += 1;
            }
            (*p2).take()
        }
    }
}