use crate::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        // 方法1
        // 创建计数器count = 0,
        // 创建一个101长度的数组arr,初始值为0，迭代nums
        // 以nums[i]为索引，将里面的值加到count上，count += arr[nums[i]]
        // 然后将arr[nums[i]]增加1
        // 一直到迭代结束，返回count即是好数对的结果
        0
    }
}