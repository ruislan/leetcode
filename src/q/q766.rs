use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        // 方法1
        // 通过观察矩阵，我们可以看到，
        // 如果我们从第一行开始向下对比，那么其实每次比较的两行的位置都是
        // 第N-1行：matrix[n-1][0..cols.len()-1]
        // 第N行：matrix[n+1][1..cols.len()]
        // 例如：1,2,3,4
        //      5,1,2,3
        //      9,5,1,2
        // 迭代1: [_1,_2,_3,4] vs [5, _1,_2,_3]
        // 迭代2: [_5,_1,_2,3] vs [9, _5,_1,_2]
        // Passed 0ms 2mb
        for row in 1..matrix.len() {
            for col in 0..matrix[row].len() - 1 {
                if matrix[row - 1][col] != matrix[row][col + 1] { return false; }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1, 2]]), true);
    assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]), false);
    assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 2, 1, 3]]), false);
    assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]), true);
}