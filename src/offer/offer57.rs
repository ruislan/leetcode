use crate::offer::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 方法1
        // 注意是一个递增的数组，所以我们可以采用两边逼近的方式，
        // 左右两个指针，令le = 0，ri = nums.len() - 1，
        // 当 le < ri的时候，
        // 如果nums[ri] > target，那么ri -= 1，继续循环
        // 如果nums[ri] <= target，
        // 如果nums[le] + nums[ri] > target，说明ri还是很大，那么ri -= 1，继续循环
        // 否则如果nums[le] + nums[ri] < target，说明le还比较小，那么le += 1，继续循环
        // 否则nums[le] + nums[ri] == target，那么就找到了这两个数字，返回这两个数
        // 当le >= ri，循环结束，如果都没找到，那么返回空数组
        vec![]
    }
}