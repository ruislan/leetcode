use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 定位x_begin_y_begin ,x_begin_y_end ,x_end_y_begin,x_end_y_end四个点
        // 因为是正方形的矩阵，所以begin和end两个参数即可
        // 当begin < end的时候，循环
        // 四个点的值相加，然后begin += 1, end -= 1
        // 循环结束
        // 如果begin == end，说明四个点在中心点汇合，则再加上中心点
        // Passed 0ms 2.1mb
        // let (mut begin, mut end, mut sum) = (0, mat.len() - 1, 0);
        // while begin < end {
        //     sum += mat[begin][begin] + mat[end][begin] + mat[begin][end] + mat[end][end];
        //     begin += 1;
        //     end -= 1;
        // }
        // if begin == end { sum += mat[begin][begin]; }
        // sum

        // 方法2
        // 递归
        // Passed 0ms 2.2mb
        fn calculate(mat: &Vec<Vec<i32>>, begin: usize, end: usize) -> i32 {
            if begin == end {
                mat[begin][begin]
            } else if begin < end {
                calculate(mat, begin + 1, end - 1) + mat[begin][begin] + mat[end][begin] + mat[begin][end] + mat[end][end]
            } else { 0 }
        }
        calculate(&mat, 0, mat.len() - 1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    assert_eq!(Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 25);
    assert_eq!(Solution::diagonal_sum(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]]), 8);
}