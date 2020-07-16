use crate::Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        // 方法1
        // 循环n从0开始，n.pow(4)
        // 如果结果大于num，break，返回false
        // 如果结果等于num,返回true

        // 方法2
        // log 4为底，放入num，结果是整数返回true，否则false
        false
    }
}