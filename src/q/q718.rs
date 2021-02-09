use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        // 方法1
        // 典型的动态规划
        // 这个题和两个字符串最长公共子串是一样的
        let mut answer = 0;
        let mut dp = vec![vec![0; b.len()]; a.len()];
        for i in 0..a.len() {
            for j in 0..b.len() {
                dp[i][j] = if i > 0 && j > 0 { dp[i - 1][j - 1] } else { 0 } + if a[i] == b[j] { 1 } else { 0 };
                answer = answer.max(dp[i][j]);
            }
        }
        answer
    }
}