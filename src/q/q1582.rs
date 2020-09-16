use crate::q::Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 暴力法，迭代mat[row][col]，然后0..rows.len()的和和0..col.len()的和是否为1
        // 是则结果增加1 O(n**2)
        // Passed 0ms 2.1mb
        // let mut count = 0;
        // for i in 0..mat.len() {
        //     for j in 0..mat[i].len() {
        //         if mat[i][j] == 1 && mat[i].iter().sum::<i32>() == 1 && mat.iter().map(|cols| cols[j]).sum::<i32>() == 1 {
        //             count += 1;
        //         }
        //     }
        // }
        // count

        // 方法2
        // 观察矩阵可以得出，可以先将每行的和计算出来，得到rows数组
        // 然后将每列的和计算出来，得到cols数组，
        // 然后统计mat[row][col]和rows和cols数组中相同位置都为1的数量即为结果
        // Passed 0ms 2.1mb
        if mat.is_empty() { return 0; }
        let rows = mat.iter().map(|rows| rows.iter().sum()).collect::<Vec<i32>>();
        let cols = (0..mat[0].len()).map(|i| mat.iter().map(|col| col[i]).sum()).collect::<Vec<i32>>();
        (0..mat.len()).fold(0, |acc, i| acc + (0..mat[i].len()).filter(|&j| mat[i][j] == 1 && rows[i] == 1 && cols[j] == 1).count()) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_special(vec![vec![]]), 0);
    assert_eq!(Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]), 1);
    assert_eq!(Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
    assert_eq!(Solution::num_special(vec![vec![0, 0, 0, 1], vec![1, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]), 2);
    assert_eq!(Solution::num_special(vec![vec![0, 0, 1, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 1, 0, 0]]), 2);
    assert_eq!(Solution::num_special(vec![vec![0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 1, 1]]), 3);
}
