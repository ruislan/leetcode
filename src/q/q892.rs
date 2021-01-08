use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 直接检查grid上下左右
        // 如果该方向上的高度比它小，直接那么减去上面的高度就是表面积
        let mut answer = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        for row in 0..rows {
            for col in 0..rows {
                let v = grid[row][col];
                if v > 0 {
                    answer += 2;
                    answer += v - if row == 0 { 0 } else if grid[row - 1][col] < v { grid[row - 1][col] } else { v };
                    answer += v - if col == 0 { 0 } else if grid[row][col - 1] < v { grid[row][col - 1] } else { v };
                    answer += v - if row == rows - 1 { 0 } else if grid[row + 1][col] < v { grid[row + 1][col] } else { v };
                    answer += v - if col == cols - 1 { 0 } else if grid[row][col + 1] < v { grid[row][col + 1] } else { v };
                }
            }
        }
        answer
    }
}