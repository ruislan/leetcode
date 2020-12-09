use crate::q::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (rows, cols) = (m as usize, n as usize);
        let mut dp = vec![vec![0; cols]; rows];
        for row in 0..rows {
            for col in 0..cols {
                if row == 0 && col == 0 {
                    dp[row][col] = 1;
                } else {
                    let up = if row == 0 { 0 } else { dp[row - 1][col] };
                    let left = if col == 0 { 0 } else { dp[row][col - 1] };
                    dp[row][col] = up + left;
                }
            }
        }
        dp[rows - 1][cols - 1]
    }
}