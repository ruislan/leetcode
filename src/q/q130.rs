use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        // 方法1
        // 利用dfs搜索row=0,row=rows - 1,和col=0,col=cols-1的联通的点
        // 也即是与四周联通的点，然后这些点都是不能改成"x"的，其他的都改成“x”
        // Passed 20ms 4.6mb
        if board.len() < 2 || board[0].len() < 2 { return; }

        let mut visited = std::collections::HashSet::new();
        let rows = board.len();
        let cols = board[0].len();

        let mut check_connected = |row: usize, col: usize| {
            if board[row][col] == 'O' && !visited.contains(&(row, col)) {
                let mut stack = vec![(row, col)];
                while !stack.is_empty() {
                    let (row, col) = stack.pop().unwrap();
                    if visited.insert((row, col)) {
                        if row < rows - 1 && board[row + 1][col] == 'O' { stack.push((row + 1, col)); }
                        if row > 0 && board[row - 1][col] == 'O' { stack.push((row - 1, col)); }
                        if col < cols - 1 && board[row][col + 1] == 'O' { stack.push((row, col + 1)); }
                        if col > 0 && board[row][col - 1] == 'O' { stack.push((row, col - 1)); }
                    }
                }
            }
        };

        for row in 0..rows {
            check_connected(row, 0);
            check_connected(row, cols - 1);
        }
        for col in 0..cols {
            check_connected(0, col);
            check_connected(rows - 1, col);
        }

        for row in 0..rows {
            for col in 0..cols {
                if !visited.contains(&(row, col)) {
                    board[row][col] = 'X';
                }
            }
        }
    }
}