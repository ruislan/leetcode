use crate::Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        // 方法1
        // 将target和arr排序，然后返回两个数组是否相等

        // 方法2
        // 构建长度为1001的数组res，将target元素作为下标设置res[i]的值为1
        // 迭代arr，将arr[i]作为下标，如果res[arr[i]]值不为1，返回false
        // 返回true
    }
}