use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        // 方法1
        // let mut nums_sorted = nums.clone();
        // nums_sorted.sort();
        // let mut min = -1i32;
        // let mut max = -1i32;
        // for i in 0..nums.len() {
        //     if nums[i] != nums_sorted[i] {
        //         if min == -1 { min = i as i32; }
        //         else { max = i as i32; }
        //     }
        // }
        // if max > 0 { max - min + 1 } else { 0 }
        
        // 方法2
        let mut stack = Vec::new();
        let mut left = nums.len();
        let mut right = 0;
        for i in 0..nums.len() {
            while !stack.is_empty() && nums[*stack.last().unwrap()] > nums[i] {
                left = std::cmp::min(left, stack.pop().unwrap());
            }
            stack.push(i);
        }
        stack.clear();
        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
                right = std::cmp::max(right, stack.pop().unwrap());
            }
            stack.push(i);
        }

        if right > left { (right - left + 1) as i32 } else { 0 }
    }
}