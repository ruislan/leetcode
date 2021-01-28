use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // 方法1
        // 分别求出左到右和右到左的前缀和
        // 前缀和相等之处就是答案
        // 否则就没有答案
        // AC 0ms 2.2mb
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