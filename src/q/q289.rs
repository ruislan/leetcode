use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        // 方法1
        // 将当前状态记录下来
        // 然后依次按照条件遍历每一个细胞
        // 根据条件修改board
        // Passed 0ms 2mb
        // if board.is_empty() { return; }
        //
        // let rows = board.len();
        // let cols = board[0].len();
        // let mut origin = board.clone();
        //
        // for row in 0..rows {
        //     for col in 0..cols {
        //         let mut lives = 0;
        //         lives += if row > 0 { origin[row - 1][col] } else { 0 };
        //         lives += if row > 0 && col > 0 { origin[row - 1][col - 1] } else { 0 };
        //         lives += if row > 0 && col < cols - 1 { origin[row - 1][col + 1] } else { 0 };
        //         lives += if col > 0 { origin[row][col - 1] } else { 0 };
        //         lives += if col < cols - 1 { origin[row][col + 1] } else { 0 };
        //         lives += if row < rows - 1 { origin[row + 1][col] } else { 0 };
        //         lives += if row < rows - 1 && col > 0 { origin[row + 1][col - 1] } else { 0 };
        //         lives += if row < rows - 1 && col < cols - 1 { origin[row + 1][col + 1] } else { 0 };
        //
        //         if lives < 2 || lives > 3 { board[row][col] = 0; }
        //         if lives == 3 && board[row][col] == 0 { board[row][col] = 1; }
        //     }
        // }

        // 方法2
        // 进阶，原地移动
        // 不用额外空间，那么我们就需要用到数据本身的空间
        // 每个数字是i32的，但是它居然只用了1和0两个状态，
        // 所以我们缩小到二进制，有多少个位置可以使用！！
        // 所以，我们可以用第二个比特位存储它之后的状态，这样就有4个状态：
        // 00(0)：当前死，之后死
        // 01(1)：当前活，之后死
        // 10(2)：当前死，之后活
        // 11(3)：当前活，之后活
        // 全部处理完成之后，我们再将所有的数字向右移动一位即可
        if board.is_empty() { return; }

        let rows = board.len();
        let cols = board[0].len();

        for row in 0..rows {
            for col in 0..cols {
                let mut lives = 0;
                lives += if row > 0 { board[row - 1][col] & 1 } else { 0 };
                lives += if row > 0 && col > 0 { board[row - 1][col - 1] & 1 } else { 0 };
                lives += if row > 0 && col < cols - 1 { board[row - 1][col + 1] & 1 } else { 0 };
                lives += if col > 0 { board[row][col - 1] & 1 } else { 0 };
                lives += if col < cols - 1 { board[row][col + 1] & 1 } else { 0 };
                lives += if row < rows - 1 { board[row + 1][col] & 1 } else { 0 };
                lives += if row < rows - 1 && col > 0 { board[row + 1][col - 1] & 1 } else { 0 };
                lives += if row < rows - 1 && col < cols - 1 { board[row + 1][col + 1] & 1 } else { 0 };

                // 这一个if段不要也是OK的，写出来只是表示一个思路的过程
                // if lives < 2 || lives > 3 {
                //     board[row][col] = (0 << 1) | board[row][col];
                // }

                if ((lives == 2 || lives == 3) && (board[row][col] & 1) == 1)
                    || (lives == 3 && (board[row][col] & 1) == 0) {
                    board[row][col] = (1 << 1) | board[row][col];
                }
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                board[row][col] >>= 1;
            }
        }
    }
}