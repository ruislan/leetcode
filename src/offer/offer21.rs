use crate::offer::Solution;

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 创建两个数组odd和even，迭代nums
        // 如果nums[i]是奇数push进odd，如果是偶数push进even
        // 然后将even数组append到odd数组后面即可

        // 方法2
        // 采用双指针le和ri
        // while当le小于ri，循环le找到偶数，循环ri找到奇数，
        // 如果le < ri，交换le和ri的数，使得le += 1， ri -= 1
        // 返回nums
        nums
    }
}