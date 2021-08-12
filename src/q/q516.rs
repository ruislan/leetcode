use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        // 方法1
        // 动态规划
        // AC 24ms 5.6mb
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; n]; n];
        for k in 1..=n {
            for i in 0..=(n - k) {
                let j = k + i - 1;
                if i == j {
                    dp[i][j] = 1;
                    continue;
                }
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[0][n - 1]
    }
}