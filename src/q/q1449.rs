use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        // 方法1
        // 还是个动态规划的典型题目
        // 这里我们核心要考虑的是两个数字比较，谁大？
        // 这个比较必然是两个：
        // 1。 谁的位数多谁大，100比99大
        // 2。 同位数，高位到低位，谁的数字大，谁大，99比89大
        // 基于这个思想，我们首先用动态规划求出长度，
        // 然后再从cost.len()到1，找出高位到低位的最大的数字
        // AC 0ms 2mb
        let mut dp = vec![i32::MIN; target as usize + 1];
        dp[0] = 0;
        for i in 1..=9 {
            let u = cost[i - 1];
            for j in u..target {
                dp[j as usize] = dp[j as usize].max(dp[(j - u) as usize] + 1);
            }
        }
        if dp[target as usize] < 0 { return String::from("0"); }
        let mut answer = String::new();
        let mut i = 9;
        let mut j = target as usize;
        while i > 0 {
            let u = cost[i - 1] as usize;
            while j >= u && dp[j] == dp[j - u as usize] + 1 {
                answer.push(char::from(b'0' + i as u8));
                j -= u;
            }
            i -= 1;
        }
        answer
    }
}


