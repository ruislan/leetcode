use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 先检查以下m * n 是不是符合条件
        // 然后直接处理即可
        let l = original.len();
        let m = m as usize;
        let n = n as usize;
        if m * n != l { return vec![]; }
        let mut ans = vec![vec![0; n as usize]; m as usize];
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = original[i * n + j];
            }
        }
        ans
    }
}