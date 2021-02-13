use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // 方法1
        // 滑动窗口解决
        // 当和< target的时候，扩展窗口
        // 当刚好>=target的时候，将窗口的长度(hi - lo + 1)与记录比较，取最小的
        // 然后将窗口左边收缩，一直到刚好< target
        // AC 0ms 2.5mb
        let n = nums.len();
        let mut answer = n + 1;
        let mut lo = 0;
        let mut sum = 0;
        for hi in 0..n {
            sum += nums[hi];
            while sum >= target {
                answer = answer.min(hi - lo + 1);
                sum -= nums[lo];
                lo += 1;
            }
        }
        if answer == n + 1 { 0 } else { answer as i32 }
    }
}