use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        // 方法1
        // 贪心算法
        // 排序之后，两个一对，取最小的那个累加
        // AC 8~12ms 2.2mb
        let mut nums = nums;
        nums.sort_unstable();
        nums.into_iter().step_by(2).sum()
    }
}