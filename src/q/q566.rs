use crate::q::Solution;

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 直接将矩阵先拉成一行
        // 然后根据R和C重新填充即可
        // 注意：如果具有给定参数的reshape操作是可行且合理的，则输出新的重塑矩阵；否则，输出原始矩阵。
        // 也即是r * c 应该等于 nums.size
        // Passed 4ms 2.1mb
        // if (r * c) as usize != nums[0].len() * nums.len() {
        //     nums
        // } else {
        //     nums.into_iter().flatten().collect::<Vec<i32>>().chunks(c as usize).map(|x| x.to_vec()).collect()
        // }

        // 方法2
        // 假设没有很方便的库函数
        // Passed 4ms 2.1mb
        // let (r, c) = (r as usize, c as usize);
        // let len = nums.len() * nums[0].len();
        // if len != r * c { return nums; }
        // let mut flatten = vec![0; len];
        // for i in 0..nums.len() {
        //     for j in 0..nums[i].len() {
        //         flatten[i * nums[i].len() + j] = nums[i][j];
        //     }
        // }
        // let mut res = vec![vec![0; c]; r];
        // for i in 0..flatten.len() {
        //     res[i / c][i % c] = flatten[i];
        // }
        // res

        // 方法3
        // 方法2重构，
        // 观察发现，其实我们可以省略掉中间转换步骤，直接设置即可
        // Passed 4ms 2.3mb
        let (r, c) = (r as usize, c as usize);
        let len = nums.len() * nums[0].len();
        if len != r * c { return nums; }
        let mut res = vec![vec![0; c]; r];
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                let pos = i * nums[i].len() + j;
                res[pos / c][pos % c] = nums[i][j];
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 3), vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4), vec![vec![1, 2, 3, 4]]);
    assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 2), vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 4, 1), vec![vec![1], vec![2], vec![3], vec![4]]);
    assert_eq!(Solution::matrix_reshape(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]], 2, 5), vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]]);
}