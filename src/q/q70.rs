use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 { return 1; }
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }
}