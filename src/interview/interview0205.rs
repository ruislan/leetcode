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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // 这个就直接迭代l1,l2，然后相加，注意进位，然后注意l1和l2的长度不一样就行了
        // AC 8ms 2.2mb
        let mut head = Some(Box::new(ListNode { val: 0, next: None }));

        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut p = head.as_mut();
        let mut carry = 0;

        while p1.is_some() && p2.is_some() {
            let x1 = p1.as_ref().unwrap().val;
            let x2 = p2.as_ref().unwrap().val;
            let sum = x1 + x2 + carry;
            carry = sum / 10;
            p.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum % 10, next: None }));
            p = p.unwrap().next.as_mut();
            p1 = p1.as_ref().unwrap().next.as_ref();
            p2 = p2.as_ref().unwrap().next.as_ref();
        }
        while p1.is_some() {
            let x = p1.as_ref().unwrap().val;
            let sum = x + carry;
            carry = sum / 10;
            p.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum % 10, next: None }));
            p = p.unwrap().next.as_mut();
            p1 = p1.as_ref().unwrap().next.as_ref();
        }
        while p2.is_some() {
            let x = p2.as_mut().unwrap().val;
            let sum = x + carry;
            carry = sum / 10;
            p.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum % 10, next: None }));
            p = p.unwrap().next.as_mut();
            p2 = p2.as_ref().unwrap().next.as_ref();
        }
        if carry > 0 {
            p.as_mut().unwrap().next = Some(Box::new(ListNode { val: carry, next: None }));
            p = p.unwrap().next.as_mut();            
        }
        
        head.unwrap().next
    }
}