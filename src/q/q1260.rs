use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // 有前提条件，50 >= grid.len >= 1, 50 >= grid[i].len >= 1
        // 方法1，根据公式 (n_cur_position + k) % n = n_new_position得到新的列地址
        // 根据公式 m_cur_position + ((k + n_cur_position) / n) % m = m_new_position得到新的行地址
        // 构建一个全新的grid(matrix)，迭代grid，根据上述公式计算出新的位置后进行填入
        // let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
        // let mut res = vec![vec![0; n]; m];
        // for i in 0..grid.len() {
        //     for j in 0..grid[i].len() {
        //         res[(i + ((k + j) / n)) % m][(j + k) % n] = grid[i][j];
        //     }
        // }
        // res

        // 方法2，拉平grid之后，再移动k % (m * n)次，然后再重组即可。
        // 例如 [[1, 2, 3],[4, 5, 6],[7, 8, 9]]，拉平即可得到1,2,3,4,5,6,7,8,9
        // 然后移动k=1次，得到9,1,2,3,4,5,6,7,8,重组之后是[9,1,2][3,4,5][6,7,8]
        let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
        let mut flat = grid.into_iter().flatten().collect::<Vec<i32>>();
        flat.rotate_right(k % (m * n));
        flat.chunks(n).map(|v| v.to_vec()).collect()
    }
}

#[test]
fn test_q1260() {
    // assert_eq!(vec![vec![]], shift_grid(vec![vec![]], ));
    assert_eq!(vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]], Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1));
    assert_eq!(vec![vec![12, 0, 21, 13], vec![3, 8, 1, 9], vec![19, 7, 2, 5], vec![4, 6, 11, 10]], Solution::shift_grid(vec![vec![3, 8, 1, 9], vec![19, 7, 2, 5], vec![4, 6, 11, 10], vec![12, 0, 21, 13]], 4));
}