use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        // 方法1
        // 这题很经典
        // 这里的中心思想就是要保持非递减！
        // 当出现一次非递减情况时，也即是nums[i] > nums[i + 1]时
        // 很容易想到的是，如果在非递减的下一个出现了递减，我们就让
        // 下一个等于我自己也即是nums[i + 1] = nums[i]即可
        // 但是这里有个问题，就是我自己[i]才是那个没有保持队形的人呢
        // 例如 1 4 2 3，如果我们让2换成了4，那么就出现了问题
        // 所以这里我们不能只看下一个，还要看上一个
        // 也就是nums[i - 1]与nums[i + 1]比较，
        //    如果nums[i + 1] >= nums[i - 1]：
        //       那么我们就让nums[i] = nums[i + 1]
        //    否则（也就是nums[i + 1] < nums[i - 1]）：
        //       那么我们就让nums[i + 1] = nums[i]
        // 看出来了吗？也就是始终把小的那个变成大的
        // 这里还有一个情况就是，如果我是第0个，没有左边，那么只能向后一个取
        // AC 0ms 1.9mb
        let n = nums.len();
        let mut nums = nums;
        let mut replaced = 0;
        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                if i == 0 || nums[i + 1] >= nums[i - 1] {
                    nums[i] = nums[i + 1];
                } else {
                    nums[i + 1] = nums[i];
                }
                replaced += 1;
                if replaced > 1 { return false; }
            }
        }
        true
    }
}