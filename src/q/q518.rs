use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // 方法1
        // 回溯算法，列出所有可能性
        // 一般会TLE，所以我们要加上记忆化
        // AC 212ms 8.4mb
        use std::collections::HashMap;
        fn dfs(amount: i32, i: usize, coins: &Vec<i32>, memo: &mut HashMap<(i32, usize), i32>) -> i32 {
            if amount == 0 {
                return 1;
            }
            if let Some(x) = memo.get(&(amount, i)) { return *x; }
            let mut count = 0;
            for j in i..coins.len() {
                let x = coins[j];
                if amount - x >= 0 {
                    count += dfs(amount - x, j, coins, memo);
                }
            }
            memo.insert((amount, i), count);
            count
        }
        let mut coins = coins;
        coins.sort_unstable();
        dfs(amount, 0, &coins, &mut HashMap::new())

        // 方法2
        // 动态规划，
        // 这个虽然看起来是二维的，但是实际上积累只有一维度
        // 相当于我们做了n个一个变量的积累，然后n个amount位置的加起来即可
        // dp[i]中的i就表示第i个面值的组合数量（注意这里用到的是组合数量，不关注顺序）
        // 转换公式就是，dp[i] += dp[i - coin[i]]
        // 例如：【1，2，5】 【5】
        // 总和是5，那么我们有：
        //    0  1  2  3   4   5
        // 1: 1  1  1  1   1   1
        // 2:       2  2   3   3
        // 5:                  4
        // AC 0ms 2.1mb
        // let mut dp = vec![0; amount as usize + 1];
        // dp[0] = 1;
        // for coin in coins {
        //     for i in coin..=amount {
        //         let i = i as usize;
        //         dp[i] += dp[i - coin as usize];
        //     }
        // }
        // dp[amount as usize]
    }
}