use crate::q::Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 找出最短路径（路径和最小）这种典型的动态规划题了
        // 因为我们是从0，0出发到n - 1,n - 1，所以当前节点只可能从它的上方或者左方过来
        // 那么状态转移就是上方或者左方最小的那个就是我们想要的
        let mut rows = grid.len();
        let mut cols = grid[0].len();
        let mut dp = vec![vec![0; cols]; rows];

        for row in 0..rows {
            for col in 0..cols {
                if row == 0 && col == 0 {
                    dp[row][col] = grid[row][col]
                } else {
                    let up = if row == 0 { i32::MAX } else { dp[row - 1][col] };
                    let left = if col == 0 { i32::MAX } else { dp[row][col - 1] };
                    dp[row][col] = grid[row][col] + up.min(left);
                }
            }
        }

        dp[rows - 1][cols - 1]
    }
}