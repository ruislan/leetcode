use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        // 方法1
        // 首先排序
        // 然后两端依次匹配即可
        // AC 40ms 3.1mb
        let mut nums = nums;
        nums.sort_unstable();
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let mut answer = 0;
        while lo < hi {
            answer = answer.max(nums[lo] + nums[hi]);
            lo += 1;
            hi -= 1;
        }
        answer
    }
}
