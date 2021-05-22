use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // 方法1
        // 这道题就是最长公共子序列啊
        // 如果两个数组匹配的数字越多，那么自然连线就越多了
        // 例如：
        //     1 4 2
        //  1  1 1 1
        //  2  1 1 2
        //  4  1 2 2
        // AC 0ms 2.1mb
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
        for i in 0..n1 {
            for j in 0..n2 {
                if nums1[i] == nums2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }
        dp[n1][n2]
    }
}