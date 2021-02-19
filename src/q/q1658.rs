use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        // 方法1
        // 滑动窗口
        // 设置nums的总和与x的差值为sum，
        // 滑动窗口，统计窗口总和，
        // 如果窗口内部的值大于sum，我们就窗口左边向右滑动
        // 减去出窗口的值直到内部的值小于等于sum
        // 如果窗口内部的值与sum相等，说明我们找到了最小操作数
        // AC 20ms 2.8mb
        let n = nums.len();
        let mut sum = nums.iter().sum::<i32>() - x;
        if sum == 0 { return n as i32; }
        if sum < 0 { return -1; }

        let mut lo = 0;
        let mut window_sum = 0;
        let mut window_max_size = 0;
        for hi in 0..n {
            window_sum += nums[hi];
            while window_sum > sum {
                window_sum -= nums[lo];
                lo += 1;
            }
            if window_sum == sum {
                window_max_size = window_max_size.max(hi - lo + 1);
            }
        }
        if window_max_size == 0 { -1 } else { (n - window_max_size) as i32 }
    }
}