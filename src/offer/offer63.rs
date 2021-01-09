use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 方法1
        // 动态规划永远是解决当前子问题最佳到全局最佳的最优秀手段
        // 这里朴素的DP，rows就是1，因为只有一次交易
        // 而cols就是每天的价格
        // dp[row][col]就是当前这一天最好的收益
        // 状态转移就是dp[row][col] = dp[row][col - 1].max(prices[col] - prices[m] m in (0..col))
        // 而这里发现每个col都要计算(0..col)的prices[m]，
        // 所以我们只需要用一个值min来记录之前的最低价格就行了
        // 公式就变成了
        // min = min.min(prices[col])
        // dp[row][col] = dp[row][col - 1].max(prices[col] - min)
        // 而我们还可以简化，不用记录所有的cols的最大的利润
        // 因为，我们只需要那个最大的利润，所以我们只留下最大的那个利润就行了
        let mut max_profit = 0;
        let mut min = prices[0];
        for col in 1..prices.len() {
            min = min.min(prices[col]);
            max_profit = max_profit.max(prices[col] - min);
        }
        max_profit
    }
}