use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        // 方法1
        // 求出每一个位置的xor，1000的数据，暴力求解可能会超时，
        // 那么只能用前缀和的形式了
        // 我们求每一行，然后再每一列的
        // 例如： 5 2  -> 5，7 -> 5,7(5,2)
        //       1 6  -> 1,7 -> 4,0(1,6)
        // AC 60ms 11.5mb
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut matrix = matrix;
        for row in 0..rows {
            for col in 1..cols {
                matrix[row][col] ^= matrix[row][col - 1];
            }
        }
        for row in 1..rows {
            for col in 0..cols {
                matrix[row][col] ^= matrix[row - 1][col];
            }
        }
        let mut v: Vec<i32> = matrix.iter().flatten().map(|x| *x).collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v[k as usize - 1]
    }
}