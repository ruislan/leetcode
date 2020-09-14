use crate::q::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target { return mid; } else if nums[mid as usize] > target { right = mid - 1; } else { left = mid + 1; }
        }
        -1
    }
}