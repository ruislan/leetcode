use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 迭代所有row，求出cols的和，找出最大的
        // 呃，这道题可以评为LC中最为简单的几道题之一了吧？
        // Passed 0ms 2.1mb
        let mut answer = 0;
        for account in accounts {
            answer = answer.max(account.into_iter().sum::<i32>());
        }
        answer
    }
}