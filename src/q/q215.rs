use crate::q::Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // let mut nums = nums;
        // nums.sort_by(|a, b| b.cmp(a));
        // nums[k as usize - 1]
        // 方法2
        let mut heap = std::collections::BinaryHeap::new();
        for n in nums {
            heap.push(n);
        }
        for _ in 0..k - 1 {
            heap.pop();
        }
        *heap.peek().unwrap()
    }
}