use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        // 方法1
        // 将当前状态记录下来
        // 然后依次按照条件遍历每一个细胞
        // 根据条件修改board
        if board.is_empty() { return; }

        let rows = board.len();
        let cols = board[0].len();
        let mut origin = board.clone();

        for row in 0..rows {
            for col in 0..cols {
                let mut lives = 0;
                lives += if row > 0 { origin[row - 1][col] } else { 0 };
                lives += if row > 0 && col > 0 { origin[row - 1][col - 1] } else { 0 };
                lives += if row > 0 && col < cols - 1 { origin[row - 1][col + 1] } else { 0 };
                lives += if col > 0 { origin[row][col - 1] } else { 0 };
                lives += if col < cols - 1 { origin[row][col + 1] } else { 0 };
                lives += if row < rows - 1 { origin[row + 1][col] } else { 0 };
                lives += if row < rows - 1 && col > 0 { origin[row + 1][col - 1] } else { 0 };
                lives += if row < rows - 1 && col < cols - 1 { origin[row + 1][col + 1] } else { 0 };

                if lives < 2 || lives > 3 { board[row][col] = 0; }
                if lives == 3 && board[row][col] == 0 { board[row][col] = 1; }
            }
        }
    }
}