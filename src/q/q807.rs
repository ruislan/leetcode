use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 求出行和列的最大值，然后迭代每个单元，取当前单元行列最大值的最小值
        // AC 0ms 2.3mb 133/133
        let mut n = grid.len();
        let mut m = grid[0].len();
        let mut row_max = vec![0; n];
        let mut col_max = vec![0; m];
        for row in 0..n {
            row_max[row] = *grid[row].iter().max().unwrap();
        }
        for col in 0..m {
            let mut max = 0;
            for row in 0..n {
                max = max.max(grid[row][col]);
            }
            col_max[col] = max;
        }
        let mut ans = 0;
        for row in 0..n {
            for col in 0..m {
                ans += row_max[row].min(col_max[col]) - grid[row][col];
            }
        }
        ans
    }
}