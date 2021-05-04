use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        // 方法1
        // dfs加记忆化，这个后面来补

        // 方法2
        // dp
        // dp[row][col]表示当前位置是否能到达，其中row表示位置，col表示步数
        // AC 40ms 5.7mb
        let n = stones.len();
        let mut dp = vec![vec![false; n]; n];
        dp[0][1] = true;
        for i in 1..n {
            for j in 0..i {
                if stones[i] - stones[j] > 0 {
                    let diff = (stones[i] - stones[j]) as usize;
                    if diff >= n || !dp[j][diff] { continue; }
                    dp[i][diff] = true;
                    if diff > 0 { dp[i][diff - 1] = true; }
                    if diff + 1 < n { dp[i][diff + 1] = true; }
                }
            }
        }
        for i in 0..n {
            if dp[n - 1][i] { return true; }
        }
        false
    }
}