use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        // 方法1
        // let mut nums = nums;
        // nums.sort();
        // let mut count = 0;
        // for i in 1..nums.len() {
        //     count += (nums[i] - nums[0]).abs();
        // }
        // count

        // 方法2
        let mut min = nums[0];
        let mut count = 0i64;
        for i in 0..nums.len() {
            count += nums[i] as i64;
            if nums[i] < min {
                min = nums[i];
            }
        }
        (count - min as i64 * nums.len() as i64) as i32
    }
}