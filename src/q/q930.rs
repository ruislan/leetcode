use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        // 方法1
        // 很粗的一个方法，后面来优化
        // AC 232ms 2mb
        let n = nums.len();
        let mut lo = 0;
        let mut sum = 0;
        let mut answer = 0;
        for hi in 0..n {
            sum += nums[hi];
            while sum > goal && lo < hi {
                sum -= nums[lo];
                lo += 1;
            }
            let mut window_sum = sum;
            let mut i = lo;
            while window_sum == goal && i <= hi {
                answer += 1;
                window_sum -= nums[i];
                i += 1;
            }
        }
        answer
    }
}