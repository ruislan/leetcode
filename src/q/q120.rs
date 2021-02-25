use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 动态规划的入门基础，最小路径和
        // 只不过这里是三角形，只能得到上面和上面左面的值的最小值来转换即可
        // AC 0ms 2.2mb
        let mut rows = triangle.len();
        let mut dp = triangle;
        for row in 1..rows {
            for col in 0..(row + 1) {
                let up = if col < row { dp[row - 1][col] } else { i32::MAX };
                let up_left = if row > 0 && col > 0 { dp[row - 1][col - 1] } else { i32::MAX };
                dp[row][col] += up.min(up_left);
            }
        }
        *dp[rows - 1].iter().min().unwrap()
    }
}