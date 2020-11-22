use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 方法1
        // 创建一个新的矩阵answer来存储平均值结果
        // 迭代矩阵m，找出当前节点周围8个点的值，求平均值即可
        // 这里向下舍入，所以直接整形除就可以了
        // 注意一下周围不足8个点的点
        // Passed 24ms 2.5mb
        let mut answer = vec![vec![0; m[0].len()]; m.len()];
        for row in 0..m.len() {
            for col in 0..m[row].len() {
                let (mut sum, mut count) = (m[row][col], 1);
                // left
                if col > 0 {
                    sum += m[row][col - 1];
                    count += 1;
                    // left up
                    if row > 0 {
                        sum += m[row - 1][col - 1];
                        count += 1;
                    }
                    // left down
                    if row < m.len() - 1 {
                        sum += m[row + 1][col - 1];
                        count += 1;
                    }
                }
                // right
                if col < m[row].len() - 1 {
                    sum += m[row][col + 1];
                    count += 1;
                    // right up
                    if row > 0 {
                        sum += m[row - 1][col + 1];
                        count += 1;
                    }
                    // right down
                    if row < m.len() - 1 {
                        sum += m[row + 1][col + 1];
                        count += 1;
                    }
                }
                // up
                if row > 0 {
                    sum += m[row - 1][col];
                    count += 1;
                }
                // down
                if row < m.len() - 1 {
                    sum += m[row + 1][col];
                    count += 1;
                }
                answer[row][col] = sum / count;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::image_smoother(
        vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1]
        ]),
               vec![
                   vec![0, 0, 0],
                   vec![0, 0, 0],
                   vec![0, 0, 0]
               ]
    )
}