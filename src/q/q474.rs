use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // 方法1
        // 这个题可以转换成一个背包问题
        // 也就是在有限的m和n的空间里面，能够装最大的数量的东西，每个东西有个两纬的重量
        // 这么一看就该用动态规划了
        // 那么dp[i][j]就是当前这个位置装的最多的东西，而我们最终结果肯定就是dp[m][n]了
        // 那么状态如何转移，实际上我们是要看dp[i - zeros][j - ones]最大装了多少个，
        // 然后+1和当前这个位置本身有多少的最大的那个，即是当前的最优解
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for str in strs {
            let mut count = (0, 0);
            for ch in str.chars() {
                if ch == '0' {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }
            for i in (count.0..=m).rev() {
                for j in (count.1..=n).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - count.0][j - count.1]);
                }
            }
        }
        dp[m][n]
    }
}