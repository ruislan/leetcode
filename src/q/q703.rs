use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    nums: BinaryHeap<Reverse<i32>>,
    k: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth = KthLargest {
            nums: BinaryHeap::new(),
            k,
        };
        for n in nums {
            kth.add(n);
        }
        kth
    }

    fn add(&mut self, val: i32) -> i32 {
        let nums = &mut self.nums;
        let k = self.k as usize;
        if nums.len() < k { nums.push(Reverse(val)); } else if nums.peek().unwrap().0 < val {
            nums.pop();
            nums.push(Reverse(val));
        }
        nums.peek().unwrap().0
    }
}