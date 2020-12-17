use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 名字与q121相同，更改为max_profit_ii
    pub fn max_profit_iii(prices: Vec<i32>, fee: i32) -> i32 {
        // 方法1
        // 动态规划
        // 动态规划三点：
        // 1.搞清楚col和row分别代表什么，这一步对了，后面就顺了
        // 2.搞清楚dp[row][col]代表什么
        // 3.搞清楚从前一个dp到这个dp[row][col]怎么来的（状态转移方程）
        //
        // 1.
        // 当你每日结算利润的时候，你会计算持仓利润和空仓利润
        // 而每日的股票价格是随机的，那么这里我们就清楚了
        // row是每日的价格，而col是持仓和空仓的利润
        // 2.
        // dp[row][0]就代表了我空仓最大利润
        // dp[row][1]就代表了我持仓最大利润
        // 因为持仓的时候，利润还有部分没有结算，那么总有dp[row][0] > dp[row][1]
        // 也即是说，最大利润就是空仓的状态
        // 3.
        // 计算空仓利润，那么可以计算昨天的空仓利润和昨天持仓后今天卖掉再
        // 减去手续费的利润中的最大值
        // 同样，计算持仓利润，可以计算昨天的空仓和今日持仓的利润与昨日持仓今天
        // 继续持有的利润中的最大值
        // Passed 40ms 4.7mb
        // let rows = prices.len();
        // let mut dp = vec![vec![0, 0]; rows + 1];
        // dp[0][1] = -prices[0];
        // for row in 1..rows {
        //     dp[row][0] = dp[row - 1][0].max(dp[row - 1][1] + prices[row] - fee);
        //     dp[row][1] = dp[row - 1][1].max(dp[row - 1][0] - prices[row]);
        // }
        // dp[rows - 1][0]

        // 方法2
        // 优化成两个参数进行
        // Passed 12ms 2.6mb
        let rows = prices.len();
        let mut profit_sell = 0;
        let mut profit_buy = -prices[0];
        for row in 1..rows {
            profit_sell = profit_sell.max(profit_buy + prices[row] - fee);
            profit_buy = profit_buy.max(profit_sell - prices[row]);
        }
        profit_sell
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit_iii(vec![1, 3, 2, 8, 4, 9], 2), 8)
}