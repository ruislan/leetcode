use crate::q::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[allow(unused)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(unused)]
impl Solution {
    // 名字重复了，更名成add_two_numbers_ii
    pub fn add_two_numbers_ii(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 简单来说就是将两个list分别存入a,b两个数组
        // 然后利用a,b进行计算到c数组里面，然后翻转c
        // 最后用c的数据生成链表就行
        // AC 0ms 1.9mb 1563/1563
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut ptr = &l1;
        while let Some(node) = ptr {
            a.push(node.val);
            ptr = &node.next;
        }
        let mut ptr = &l2;
        while let Some(node) = ptr {
            b.push(node.val);
            ptr = &node.next;
        }
        let mut n = a.len().min(b.len());
        let mut i = 0;
        let mut carry = 0;
        let mut c = Vec::new();
        while i < n {
            let sum = a.pop().unwrap() + b.pop().unwrap() + carry;
            carry = sum / 10;
            c.push(sum % 10);
            i += 1;
        }
        while let Some(val) = a.pop() { 
            let sum = val + carry;
            carry = sum / 10;     
            c.push(sum % 10);       
        }
        while let Some(val) = b.pop() {
            let sum = val + carry;
            carry = sum / 10;     
            c.push(sum % 10);
        }
        if carry > 0 { c.push(carry); }
        c.reverse();


        let mut head = Some(Box::new(ListNode { val: 0, next:None }));
        let mut ptr = &mut head;
        for val in c.into_iter() {
            ptr.as_mut().unwrap().next = Some(Box::new(ListNode { val, next:None }));
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        head.unwrap().next

        // 方法2
        // 还是a和b存储数据，不过这里我们不用c数组，而是直接操作
        // 每次计算结果都生成一个新的链表头，然后接续上之前的链表就行
        // 这里代码我就不写了
    }
}