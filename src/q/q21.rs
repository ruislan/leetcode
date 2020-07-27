mod q21 {
    // Definition for singly-linked list.
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

    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 方法1
        // let mut res = Box::new(ListNode::new(0));
        // let mut prev = &mut res;
        // while l1 != None && l2 != None {
        //     let n1 = l1.as_ref().unwrap().val;
        //     let n2 = l2.as_ref().unwrap().val;
        //     if n1 <= n2 {
        //         prev.next = l1.clone();
        //         l1 = l1.unwrap().next;
        //     } else {
        //         prev.next = l2.clone();
        //         l2 = l2.unwrap().next;
        //     }
        //     prev = prev.next.as_mut().unwrap();
        // }
        // if l1 == None { prev.next = l2; }
        // else { prev.next = l1; }
        // res.next
        
        // 方法2
        let mut nums = Vec::new();
        loop {
            if l1 == None && l2 == None { break; }
            if l1 != None {
                nums.push(l1.as_ref().unwrap().val);
                l1 = l1.unwrap().next;
            }
            if l2 != None {
                nums.push(l2.as_ref().unwrap().val);
                l2 = l2.unwrap().next;
            }
        }
        nums.sort();
        let mut result = Box::new(ListNode::new(0));
        let mut cur_node = &mut result;
        for num in nums {
            cur_node.next = Some(Box::new(ListNode::new(num)));
            cur_node = cur_node.next.as_mut().unwrap();
        }
        result.next
    }
}