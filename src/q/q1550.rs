use crate::q::Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        // 方法1
        // 创建奇数计数器来计算连续的奇数数量odds
        // 判断奇数的方法是 n & 1 == 1
        // 迭代arr，如果当前是奇数，就令odds += 1
        // 如果奇数计数到3，则直接返回true
        // 迭代结束，返回false
        false
    }
}