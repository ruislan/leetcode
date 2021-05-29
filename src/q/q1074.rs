use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        // 方法1
        // 1. 先求出横向和纵向的前缀和
        // 2. 求出每一个位置的符合条件的数量
        // O(n^4)
        // TLE 超时
        // let mut rows = matrix.len();
        // let mut cols = matrix[0].len();
        // let mut prefix_sum = vec![vec![0; cols + 1]; rows];
        // for row in 0..rows {
        //     for col in 0..cols {
        //         prefix_sum[row][col + 1] = prefix_sum[row][col] + matrix[row][col];
        //     }
        // }
        // let mut answer = 0;
        // for row in 0..rows {
        //     for col in 0..cols {
        //         for i in (0..=row).rev() {
        //             let s_t_col = (i..=row).fold(0, |acc, k| acc + prefix_sum[k][col + 1]);
        //             if s_t_col == target { answer += 1; }
        //             for j in 0..col {
        //                 let s_col = (i..=row).fold(0, |acc, k| acc + prefix_sum[k][j + 1]);
        //                 if s_t_col - s_col == target { answer += 1; }
        //             }
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 利用hashmap优化，提前存储竖向的和，可以降低到O(n^3)
        // AC 124ms 2.2mb
        use std::collections::HashMap;
        let mut rows = matrix.len();
        let mut cols = matrix[0].len();
        let mut matrix = matrix;
        for row in 0..rows {
            for col in 1..cols {
                matrix[row][col] += matrix[row][col - 1];
            }
        }
        let mut answer = 0;
        for k in 0..cols {
            for j in k..cols {
                let mut sum = 0;
                let mut map: HashMap<i32, i32> = HashMap::new();
                map.insert(0, 1);
                for i in 0..rows {
                    sum += if k == 0 {
                        matrix[i][j]
                    } else {
                        matrix[i][j] - matrix[i][k - 1]
                    };
                    if map.contains_key(&(sum - target)) {
                        answer += *(map.entry(sum - target).or_insert(0));
                    }
                    *map.entry(sum).or_insert(0) += 1;
                }
            }
        }
        answer
    }
}