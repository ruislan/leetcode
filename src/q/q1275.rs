use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        // 方法1
        // 构建一个3X3的board，然后迭代moves，将even的点填充为'A'（对应X），将odd的点填充为'B'（对应O）
        // 首先检查3行，再检查3列，再检查对角线，
        // if 存在3个一样的，则得到是2还是1，返回胜利者
        // if moves.len() == 9，Draw
        // else 返回Pending
        let mut board = vec![vec![b' '; 3]; 3];
        for i in 0..moves.len() {
            board[moves[i][0] as usize][moves[i][1] as usize] = if i % 2 == 0 { b'A' } else { b'B' };
        }
        for i in 0..3 {
            if board[i][0] != b' ' && board[i][0] & board[i][1] == board[i][2] {
                return (board[i][0] as char).to_string();
            }
            if board[0][i] != b' ' && board[0][i] & board[1][i] == board[2][i] {
                return (board[0][i] as char).to_string();
            }
        }
        if board[1][1] != b' ' && (board[0][0] & board[1][1] == board[2][2] || board[0][2] & board[1][1] == board[2][0]) {
            return (board[1][1] as char).to_string();
        }
        if moves.len() == 9 { "Draw".to_string() } else { "Pending".to_string() }
    }
}

#[test]
fn test_q1275() {
    // assert_eq!(Solution::tictactoe(vec![vec![]]), "".to_string());
    assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]]), "A".to_string());
    assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![0, 2], vec![1, 0], vec![2, 0], vec![0, 0], vec![1, 1], vec![0, 1], vec![0, 2], vec![1, 0], vec![2, 0]]), "B".to_string());
    assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![1, 1], vec![2, 0], vec![1, 0], vec![1, 2], vec![2, 1], vec![0, 1], vec![0, 2], vec![2, 2]]), "Draw".to_string());
    assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![1, 1]]), "Pending".to_string());
}