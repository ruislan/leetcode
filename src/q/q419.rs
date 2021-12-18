use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        // 方法1
        // 计算战舰的规则是
        // 1。相连的算1艘战舰
        // 2。战舰要么横着，要么竖着，也就是说没有相交的情况
        // 这里我们可以只用迭代一次，当发现是'X'的时候，左边和上边都没有'X'的情况下，它也可以被计算
        // AC 0ms 3.8mb 27/27
        let rows = board.len();
        let cols = board[0].len();
        let mut ans = 0;
        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] == 'X' {
                    if row == 0 && col == 0 { ans += 1; }
                    if row > 0 && col > 0 && board[row - 1][col] == '.' && board[row][col - 1] == '.' { ans += 1; }
                    if row > 0 && col == 0 && board[row - 1][col] == '.' { ans += 1; }
                    if row == 0 && col > 0 && board[row][col - 1] == '.' { ans += 1; }
                }
            }
        }
        ans
    }
}