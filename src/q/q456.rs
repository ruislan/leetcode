use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        // 方法1
        // if nums.len() < 3 { return false; }
        //
        // for i in 0..nums.len() - 1 {
        //     let n = nums[i];
        //     let mut max = n;
        //     let mut min = n;
        //     for j in i + 1..nums.len() {
        //         if nums[j] > max {
        //             max = nums[j];
        //         }
        //         if nums[j] < max && nums[j] > min { return true; }
        //     }
        // }
        // false
        
        // 方法2
        if nums.len() < 3 { return false; }

        let mut min = vec![0; nums.len()];
        min[0] = nums[0];
        for i in 1..nums.len() {
            min[i] = if min[i - 1] > nums[i] { nums[i] } else { min[i - 1] }
        }

        let mut stack = Vec::new();
        for i in (0..nums.len()).rev() {
            if nums[i] > min[i] {
                while !stack.is_empty() && *stack.last().unwrap() <= min[i] {
                    stack.pop();
                }
                if !stack.is_empty() && *stack.last().unwrap() < nums[i] {
                    return true;
                }
                stack.push(nums[i]);
            }
        }

        false
    }
}