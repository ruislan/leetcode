use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // 方法1
        // 动态规划
        // dp[i][j]表示当前这个数值的情况下最少的那个方法
        // i就是完全平方数值1，4，9，16……
        // j就是1..=n的结果
        // 最后我们看dp[..][n]最小的那个
        // 例如：1 4 9 16
        // 0 1 2 3 4 5 6 7 8 9 10 11 12
        // 1 0 1 2 3 4 5 6 7 8 9 10 11 12
        // 4         1 2 3 4 2 3 4  5  3
        // 9                   1 2  3  4
        // 16 > 12 break
        // 这里我们其实可以压缩一下，不需要两个纬度
        // 用dp[i]表示即可
        // AC 32ms 2.1mb
        let mut answer = n;
        let n = n as usize;
        let mut dp = vec![answer; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            let x = i * i;
            for j in x..=n {
                dp[j] = dp[j].min(dp[j - x] + 1);
            }
            answer = answer.min(dp[n]);
        }
        answer
    }
}


