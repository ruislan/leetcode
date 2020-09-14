use crate::q::Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0i32;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    let at_left = j == 0;
                    let at_right = j == grid[i].len() - 1;
                    let at_up = i == 0;
                    let at_down = i == grid.len() - 1;

                    let mut has_left = false;
                    let mut has_right = false;
                    let mut has_up = false;
                    let mut has_down = false;

                    if !at_up { has_up = grid[i - 1][j] == 1; }
                    if !at_down { has_down = grid[i + 1][j] == 1; }
                    if !at_left { has_left = grid[i][j - 1] == 1; }
                    if !at_right { has_right = grid[i][j + 1] == 1; }

                    if !has_left { res += 1; }
                    if !has_right { res += 1; }
                    if !has_up { res += 1; }
                    if !has_down { res += 1; }
                }
            }
        }
        res
    }
}