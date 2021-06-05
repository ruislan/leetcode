use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 方法1
        // 先验证行，再验证列，再验证小3*3
        // let mut set_row = vec![bool; 9];
        // AC 4ms 2mb
        for row in 0..9 {
            let mut set_col = vec![false; 9];
            for col in 0..9 {
                if let Some(x) = board[row][col].to_digit(10) {
                    if set_col[x as usize - 1] { return false; }
                    set_col[x as usize - 1] = true;
                }
            }
        }
        for col in 0..9 {
            let mut set_row = vec![false; 9];
            for row in 0..9 {
                if let Some(x) = board[row][col].to_digit(10) {
                    if set_row[x as usize - 1] { return false; }
                    set_row[x as usize - 1] = true;
                }
            }
        }
        for h in (0..9).step_by(3) {
            let row_start = h;
            let row_end = h + 3;
            for w in (0..9).step_by(3) {
                let col_start = w;
                let col_end = w + 3;
                let mut set = vec![false; 9];
                for row in row_start..row_end {
                    for col in col_start..col_end {
                        if let Some(x) = board[row][col].to_digit(10) {
                            if set[x as usize - 1] { return false; }
                            set[x as usize - 1] = true;
                        }
                    }
                }
            }
        }
        true
    }
}