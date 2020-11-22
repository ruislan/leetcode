use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sl = 0;
        let mut sr = 0;
        for i in (0..nums.len()).rev() {
            sr += nums[i];
        }
        for i in 0..nums.len() {
            sl += nums[i];
            if sl == sr { return i as i32; }
            sr -= nums[i];
        }
        -1
    }
}