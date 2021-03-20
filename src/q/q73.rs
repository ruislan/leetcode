use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // 方法1
        // 用两个Vec分别记录要清零的rows和cols
        // 遍历matrix，遇到0的填入其rows和cols
        // 依次清零row和col
        // 空间属于O(m + n)
        // AC 4ms 2.3mb
        // let rows = matrix.len();
        // let cols = matrix[0].len();
        // let mut row_zeros = Vec::new();
        // let mut col_zeros = Vec::new();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         if matrix[row][col] == 0 {
        //             row_zeros.push(row);
        //             col_zeros.push(col);
        //         }
        //     }
        // }
        // for row in row_zeros.into_iter() {
        //     for col in 0..cols {
        //         matrix[row][col] = 0;
        //     }
        // }
        // for col in col_zeros.into_iter() {
        //     for row in 0..rows {
        //         matrix[row][col] = 0;
        //     }
        // }

        // 方法2
        // 我们可以将是否为0的数字压缩到第0行和第0列，
        // 简单来说就是从1行和1列开始迭代，如果出现了0，那么就设置这一行第0列为0和第0行为0
        // 然后我们再迭代第0行设置所有为0的列全部为0，第0列设置所有为0的那一行全部为0
        // 需要注意的是，我们需要先存储第0行和第0列是否有0的情况，这样才保证后面第0行和第0列也是否都要为0的情况
        // 例如： 1 1 1     1 0 1     1  0  1
        //       1 0 1 ->  0 0 1 ->  0  0  0
        //       1 1 1     1 1 1     1  0  1
        // 这里就只需要O(1)的额外空间了，因为我们只用了两个参数来记录0行和0列是否全部置0
        // AC 8ms 2.3mb
        // P.S 节约的这点空间对于Rust来说微不足道哈
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut is_row0_zero = false;
        let mut is_col0_zero = false;
        for row in 0..rows {
            if matrix[row][0] == 0 {
                is_col0_zero = true;
                break;
            }
        }
        for col in 0..cols {
            if matrix[0][col] == 0 {
                is_row0_zero = true;
                break;
            }
        }
        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][col] == 0 {
                    matrix[0][col] = 0;
                    matrix[row][0] = 0;
                }
            }
        }
        for row in 1..rows {
            if matrix[row][0] == 0 {
                for col in 0..cols {
                    matrix[row][col] = 0;
                }
            }
        }
        for col in 1..cols {
            if matrix[0][col] == 0 {
                for row in 0..rows {
                    matrix[row][col] = 0;
                }
            }
        }
        if is_row0_zero {
            for col in 0..cols {
                matrix[0][col] = 0;
            }
        }
        if is_col0_zero {
            for row in 0..rows {
                matrix[row][0] = 0;
            }
        }
    }
}
