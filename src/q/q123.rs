use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法与之前的重名，所以更新为max_profit_123
    pub fn max_profit_123(prices: Vec<i32>) -> i32 {
        // 方法1
        // 最大利润这种，动态规划没得说
        // 动态规划通常dp[row][col]就代表当前子集的最佳解法
        // 所以，这里就是两次交易最大的利润
        // 然后搞清楚rows和cols分别代表什么
        // 因为是多少次交易最大的利润，那么rows可以设置为第k次交易
        // cols就是当天的价格。
        // 最后是状态转移方程
        // 这里既然是最大利润，那么我们肯定要与昨天的最大利润dp[row - 1][col]比较，看谁大
        // 这里我们先看看只有一次交易的情况
        // 如果我们只有一次交易，那么很显然，我们的状态转移方程很好写
        // 那么就是第col天的价格与第0..=col天的价格的差值中最大的那个，再与昨天最大利润比较，取最大那个就是1次交易最大获利
        // dp[row][col] = max(dp[row][col - 1], max(prices[col] - prices[col - m](m = 0..=col)))
        // 那么现在最多2次交易的话，我们可以把dp[row][col]看成是已经完成了一次交易的情况
        // 所以我们的状态转移方程，就需要再在与那一天的差值的基础上再加上那一天已经交易的利润，这样就是最大2次交易了
        // dp[row][col] = max(dp[row][col - 1], max(dp[row - 1][col - m] + prices[col] - prices[col - m](m = 0..=col)))
        // 这里如果用最朴素的方式，数据量较大的时候会超时
        // Not Passed
        // let mut rows = 3;
        // let mut cols = prices.len();
        // let mut dp = vec![vec![0; cols]; rows];
        // for row in 1..rows {
        //     for col in 1..cols {
        //         dp[row][col] = dp[row][col - 1].max(
        //             prices[col] + (0..col).map(|m| dp[row - 1][m] - prices[m]).max().unwrap()
        //         );
        //     }
        // }
        // dp[rows - 1][cols - 1]

        // 方法2
        // 我们需要优化方法1
        // 这里我们观察到我们每次都要处理(0..col).map(|m| dp[row - 1][m] - prices[m]).max()
        // 也就是col 多1个，那么我们就要计算1次0..col的最大值
        // 所以我们的时间复杂度变成了O(n^2*k), k = 2
        // 那么实际上，我们不用每次都重复计算，只需要每多1个col，我们就看看是不是最大值max
        // 这样我们就把时间复杂度降低到了O(n*k),k = 2
        // Passed 20ms 3.7mb
        let rows = 3;
        let cols = prices.len();
        let mut dp = vec![vec![0; cols]; rows];

        for row in 1..rows {
            let mut max = -prices[0];
            for col in 1..cols {
                max = max.max(dp[row - 1][col - 1] - prices[col - 1]);
                dp[row][col] = dp[row][col - 1].max(prices[col] + max);
            }
        }

        dp[rows - 1][cols - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit_123(vec![1, 2, 3, 4, 5]), 4);
}