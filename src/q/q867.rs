use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 方法1
        // 通过观察可以看出来，转置就是依次将处于同一个col下的所有row组合成col
        // 简单来说3x3的a将a[0][0], a[1][0], a[2][0]重新作为第一个row
        // Passed 4ms 2.1mb
        // let (cols, rows) = (a.len(), a[0].len());
        // let mut answer = vec![vec![0; cols]; rows];
        // for row in 0..rows {
        //     for col in 0..cols {
        //         answer[row][col] = a[col][row];
        //     }
        // }
        // answer

        // 方法2
        // 优化一下方法1
        // 不用一开始就弄出整个空间，我们可以逐个从原矩阵中拿数据出来填入
        // Passed 0ms 2.1mb
        let mut answer = vec![Vec::new(); a[0].len()];
        for row in a {
            for (j, col) in row.into_iter().enumerate() {
                answer[j].push(col);
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::transpose(vec![vec![1]]), vec![vec![1]]);
    assert_eq!(Solution::transpose(vec![vec![1, 2]]), vec![vec![1], vec![2]]);
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
}