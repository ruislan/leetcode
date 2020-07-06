use crate::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // 方法1
        // 排序nums，然后取最后两个位置的数字进行计算(nums[i]-1)*(nums[j]-1)

        // 方法2
        // 查找nums中的最大值，和第二大的值
        // 然后代入计算(top1-1)*(top2-1)
    }
}