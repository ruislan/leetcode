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
        let n = nums.len();
        let mut sum = nums.iter().sum::<i32>() - x;
        let mut lo = 0;
        let mut hi = 0;
        let mut sub_sum = 0;
        let mut found = false;
        while hi < n {
            sub_sum += nums[hi];
            while sub_sum > sum {
                sub_sum -= nums[lo];
                lo += 1;
            }
            if sub_sum == sum {
                found = true;
                break;
            }
            hi += 1;
        }
        if found { (n - (hi - lo + 1)) as i32 } else { -1 }
    }
}