use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 模拟法
        // 从外圈开始填数字，填完一圈之后，向内缩小一圈
        // 直到正方形的四个角相等，
        // 这里就要看n是奇数还是偶数了
        // 如果n是奇数，那么说明四个角相等的位置还有一个数字
        // 如果n是偶数，那么说明四个角相等的位置已经没有数字了
        // AC 0ms 1.9mb
        let n = n as usize;
        let mut answer = vec![vec![0; n]; n];
        let (mut x_begin, mut x_end) = (0, n - 1);
        let (mut y_begin, mut y_end) = (0, n - 1);
        let mut num = 1;
        while x_begin < x_end && y_begin < y_end {
            (x_begin..=x_end).for_each(|x| {
                answer[y_begin][x] = num;
                num += 1;
            });
            (y_begin + 1..y_end).for_each(|y| {
                answer[y][x_end] = num;
                num += 1;
            });
            (x_begin..=x_end).rev().for_each(|x| {
                answer[y_end][x] = num;
                num += 1;
            });
            (y_begin + 1..y_end).rev().for_each(|y| {
                answer[y][x_begin] = num;
                num += 1;
            });
            x_begin += 1;
            x_end -= 1;
            y_begin += 1;
            y_end -= 1;
        }
        if n & 1 == 1 { answer[x_begin][y_begin] = num; }
        answer
    }
}