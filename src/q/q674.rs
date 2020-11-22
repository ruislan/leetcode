use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 { return 0; }
        let mut max = 0;
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                count += 1;
            } else {
                if count > max {
                    max = count;
                }
                count = 1;
            }
        }

        if count > max {
            max = count;
        }
        max
    }
}