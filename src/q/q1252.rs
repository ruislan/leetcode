use crate::q::Solution;

impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        // 方法一：构建一个n*m的矩阵，循环indices，得到每个点indice
        // 循环indice(0..N,Mx)和indice(Nx,0..M)，每个点都加上1
        // 循环矩阵的每个点，统计奇数个数
        let mut matrix = vec![vec![0; m as usize]; n as usize];
        for i in 0..indices.len() {
            for j in 0..n as usize {
                matrix[j][indices[i][1] as usize] += 1;
            }
            for k in 0..m as usize {
                matrix[indices[i][0] as usize][k] += 1;
            }
        }
        matrix.iter().map(|r| r.iter().filter(|&&p| p % 2 != 0).count() as i32).sum()
    }
}

#[test]
pub fn test_q1252() {
    assert_eq!(6, Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]));
    assert_eq!(0, Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]));
}