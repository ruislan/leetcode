use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // 方法1
        // 这个题只用了删除，而没有替换或者插入操作
        // 那么也就是意味着，我们只需要找出最长公共子序列（LCS）就可以了
        // 这样保证双方保留的字符是最多的剩下的就是删除的
        // 当然最小编辑距离也是可以解决的（Edit Distance)
        // AC 0ms 4mb 1306/1306
        let n = word1.len();
        let m = word2.len();
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        (n + m - dp[n][m] * 2) as i32
    }
}
