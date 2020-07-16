use crate::Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // 方法1
        // 循环n从0开始，n.pow(3)
        // 如果结果大于n，break，返回false
        // 如果结果等于n,返回true

        // 方法2
        // log 3为底，放入n，结果是整数返回true，否则false
        false
    }
}