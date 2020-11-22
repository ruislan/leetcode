use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 方法1
        // 逐行水平翻转和反转每个数字
        // Passed 0ms 2mb
        // let mut a = a;
        // a.iter_mut().for_each(|row| {
        //     row.reverse();
        //     (0..row.len()).for_each(|i| row[i] ^= 1);
        // });
        // a
        // 方法1改进
        // 一行解决
        // Passed 0ms 2mb
        a.into_iter().map(|row| row.into_iter().rev().map(|i| i ^ 1).collect()).collect()

        // 方法2
        // 逐行头尾交换，然后翻转每个数字
        // Passed 0ms 2mb
        // let mut a = a;
        // a.iter_mut().for_each(|row| {
        //     let (mut l, mut r) = (0, row.len() - 1);
        //     while l < r {
        //         row[l] ^= 1;
        //         row[r] ^= 1;
        //         row.swap(l, r);
        //         l += 1;
        //         r -= 1;
        //     }
        //     if l == r { row[l] ^= 1; }
        // });
        // a
    }
}

#[test]
fn test() {
    assert_eq!(Solution::flip_and_invert_image(
        vec![vec![1]]
    ), vec![vec![0]]);
    assert_eq!(Solution::flip_and_invert_image(
        vec![vec![1, 0], vec![1, 0]]
    ), vec![vec![1, 0], vec![1, 0]]);
    assert_eq!(Solution::flip_and_invert_image(
        vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]
    ), vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]);
    assert_eq!(Solution::flip_and_invert_image(
        vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]]
    ), vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 1, 0]]);
}