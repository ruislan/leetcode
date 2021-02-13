use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 典型的动态规划
        // dp[i][j]存储的是到当前这个位置最大的价值
        // 它是由上方和左方中的最大价值积累过来
        // AC 0ms 2.4mb
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();
        for row in 0..rows {
            for col in 0..cols {
                let up = if row > 0 { grid[row - 1][col] } else { 0 };
                let left = if col > 0 { grid[row][col - 1] } else { 0 };
                grid[row][col] += up.max(left);
            }
        }
        grid[rows - 1][cols - 1]
    }
}
