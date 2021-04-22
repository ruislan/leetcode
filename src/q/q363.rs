use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        // 方法1
        // 先求出每个row的所有col前缀和
        // 然后逐步迭代col，我们设为start_col，然后逐步增加col的长度scale，得到end_col的位置
        // 我们根据前缀和可以求出所有每个row的start_col:end_col的和
        // 然后我们再求出row的前缀和，这样我们可以在O(n)的时间里面计算出区间start_row到end_row的和
        // 这样我们就求出了各种尺寸框框的和，然后我们找出小于等于k的那个和sum
        // 如果等于k，那就直接返回
        // 如果小于k，那么sum最接近k的那个就是结果
        // AC 156ms 2mb
        let mut rows = matrix.len();
        let mut cols = matrix[0].len();
        let mut prefix_sum_cols = vec![vec![0; cols + 1]; rows];
        for row in 0..rows {
            for col in 1..=cols {
                prefix_sum_cols[row][col] += prefix_sum_cols[row][col - 1] + matrix[row][col - 1];
            }
        }

        let mut answer = i32::MIN;

        for start_col in 0..cols {
            for end_col in start_col..cols {
                let mut prefix_sum_rows = vec![0; rows + 1];
                for row in 0..rows {
                    prefix_sum_rows[row + 1] = prefix_sum_cols[row][end_col + 1] - prefix_sum_cols[row][start_col] + prefix_sum_rows[row];
                }
                for start_row in 0..rows {
                    for end_row in start_row..rows {
                        let sum = prefix_sum_rows[end_row + 1] - prefix_sum_rows[start_row];
                        if sum == k { return k; }
                        if sum < k {
                            answer = answer.max(sum);
                        }
                    }
                }
            }
        }
        answer
    }
}