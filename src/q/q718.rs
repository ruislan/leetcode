use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        // 方法1
        // 典型的动态规划
        // 这个题和两个字符串最长公共子串是一样的
        // 复习这个题的时候，注意和最长公共子序列一起复习，
        // 同时最长公共子序列最好能够推导出是哪些序列
        // AC 60ms 5.9mb
        let mut answer = 0;
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                    answer = answer.max(dp[i][j]);
                }
            }
        }
        answer

        // 方法2
        // 利用滑动窗口比较重复子数组
    }
}