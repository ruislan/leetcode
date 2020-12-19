use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        // 方法1
        // 首先找到R的位置
        // 然后以R为中心探测上下左右四个方向，遇到的第一个障碍物结束探测
        // 如果那个障碍物是p，结果就加1
        // 返回结果
        // Passed 0ms 2mb
        let (rows, cols) = (8, 8);
        let (mut r_row, mut r_col) = (0, 0);
        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] == 'R' {
                    r_row = row;
                    r_col = col;
                }
            }
        }

        let mut answer = 0;

        let mut up = r_row.overflowing_sub(1).0;
        while up < rows {
            if board[up][r_col] == 'B' {
                break;
            } else if board[up][r_col] == 'p' {
                answer += 1;
                break;
            }
            up = up.overflowing_sub(1).0;
        }

        let mut down = r_row + 1;
        while down < rows {
            if board[down][r_col] == 'B' {
                break;
            } else if board[down][r_col] == 'p' {
                answer += 1;
                break;
            }
            down += 1;
        }

        let mut left = r_col.overflowing_sub(1).0;
        while left < cols {
            if board[r_row][left] == 'B' {
                break;
            } else if board[r_row][left] == 'p' {
                answer += 1;
                break;
            }
            left = left.overflowing_sub(1).0;
        }

        let mut right = r_col + 1;
        while right < cols {
            if board[r_row][right] == 'B' {
                break;
            } else if board[r_row][right] == 'p' {
                answer += 1;
                break;
            }
            right += 1;
        }

        answer
    }
}