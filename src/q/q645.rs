use crate::q::Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut bucket = vec![0; nums.len()];
        let mut dup = 0;
        for i in 0..nums.len() {
            bucket[nums[i] as usize - 1] += 1;
            if bucket[nums[i] as usize - 1] > 1 {
                dup = nums[i];
            }
        }
        let mut lost = 0;
        for i in 0..bucket.len() {
            if bucket[i] == 0 {
                lost = (i + 1) as i32;
                break;
            }
        }
        vec![dup, lost]
    }
}