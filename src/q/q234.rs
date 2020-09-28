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
    // 方法名字与q9相同，修改名字为is_palindrome_linked_list
    pub fn is_palindrome_linked_list(head: Option<Box<ListNode>>) -> bool {
        // 方法1
        // 遍历链表，将数字取出来放入数组，然后检查数组是否是回文
        // Passed 0ms 4mb
        let mut arr = Vec::new();
        let mut head = head;
        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }
        let (mut p1, mut p2) = (0, arr.len().saturating_sub(1));
        while p1 < p2 {
            if arr[p1] != arr[p2] { return false; }
            p1 += 1;
            p2 -= 1;
        }
        true

        // 方法2
        // 不用额外的数组来存储，只有几个变量，达到O(1)级别的存储使用
        // 首先记录元素总数，然后将链表分割为前半段和后半段，将前半段或者后半段翻转，最后再比较两个链表是否相等
        // 需要注意的是元素的奇偶性，如果是奇数那么奇数点不需要参与判断，直接略过
        // Passed 4ms 3.9mb
        // let mut count = 0;
        // let mut ptr = head.as_ref();
        // while let Some(node) = ptr {
        //     count += 1;
        //     ptr = node.next.as_ref();
        // }
        //
        // if count <= 1 { return true; }
        //
        // let (mut head, mid) = (head, (count >> 1) - 1);
        // let (mut ptr, mut pos) = (head.as_mut(), 0);
        // let mut right = None;
        // while let Some(node) = ptr {
        //     if pos == mid { right = node.next.take(); }
        //     pos += 1;
        //     ptr = node.next.as_mut();
        // }
        // // 去掉奇数节点
        // if count & 1 == 1 { right = right.unwrap().next; }
        // // 翻转right
        // let mut prev = None;
        // while let Some(mut node) = right {
        //     let mut next = node.next.take();
        //     node.next = prev;
        //     prev = Some(node);
        //     right = next;
        // }
        // // 检测prev和head是否相同
        // let (mut p1, mut p2) = (head, prev);
        // while p1.is_some() && p2.is_some() {
        //     if p1.as_ref().unwrap().val != p2.as_ref().unwrap().val { return false; }
        //     p1 = p1.unwrap().next;
        //     p2 = p2.unwrap().next;
        // }
        // true

        // 方法3
        // 方法1为什么速度比方法2快，内存使用差不多，这里我做个猜想，
        // 因为Rust的所有权转移后内容被释放，所以虽然看起来是多了个数组来存储，
        // 但是实际上我们是把链表的数据转移成数组了，内存并没有增多
        // 相当于是新增一个数组元素，就销毁一个链表元素
        // 而速度更加快（非数量级），则是由于不需要翻转再比较的缘故，少了不少操作
        // 为了验证我的想法，所以有了方法3，我们用方法1的思路，但是不销毁链表看看内存
        // Passed 0ms 5.3mb
        // 果然是这样，那么我的猜想就很成功
        // let mut arr = Vec::new();
        // let mut head = head.as_ref();
        // while let Some(node) = head {
        //     arr.push(node.val);
        //     head = node.next.as_ref();
        // }
        // let (mut p1, mut p2) = (0, arr.len().saturating_sub(1));
        // while p1 < p2 {
        //     if arr[p1] != arr[p2] { return false; }
        //     p1 += 1;
        //     p2 -= 1;
        // }
        // true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_palindrome_linked_list(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None,
            })),
        }))
    ), false);
}