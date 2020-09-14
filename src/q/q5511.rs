use crate::q::Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 暴力法，迭代mat[row][col]，然后0..rows.len()的和和0..col.len()的和是否为1
        // 是则结果增加1

        // 方法2
        // 观察矩阵可以得出，可以先将每行的和计算出来，得到rows数组
        // 然后将每列的和计算出来，得到cols数组，
        // rows数组和cols数组中长度较短的那个长度用0来补齐等长
        // 然后统计rows和cols数组中相同位置都为1的即为结果
        0
    }
}