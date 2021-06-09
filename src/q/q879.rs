use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        // 方法1
        // 这种题一看就是动态规划了，就是局部最优解，积累成全局最优解的
        // 这里我们有三个要考虑的变量哈，一个是n个工作，一个是分组人数，一个是利润数
        // 所以就变成了一个三维的dp了
        // dp[k][i][j] 一般代表局部最优解，这里表示都是使用j个人员完成前k个任务去达成i个利润的数量
        // 接下来就是状态转移，
        // 首先，我们第dp[k][i][j]至少是dp[k-1][i][j]的，这表示第k个任务不做，至少数量是k-1的数量，
        // 然后，如果要做k个任务，我们要检查一下做这个任务的人数够不够，也就是j是不是>=g的
        // 否则，我们就做不了这个任务，如果可以做
        // 最后，我们统计所有在利润min_profit位置的，包含所有分组的数量
        // AC 40ms 6.2mb
        let k_mod = 1_000_000_007;
        let k_len = group.len();
        let p_len = min_profit as usize;
        let g_len = n as usize;
        let mut dp = vec![vec![vec![0; g_len + 1]; p_len + 1]; k_len + 1];
        dp[0][0][0] = 1;
        for k in 1..=k_len {
            let p = profit[k - 1];
            let g = group[k - 1] as usize;
            for i in 0..=p_len {
                for j in 0..=g_len {
                    dp[k][i][j] = (if j < g { 0 } else { dp[k - 1][0.max(i as i32 - p) as usize][j - g] }
                        + dp[k - 1][i][j]) % k_mod;
                }
            }
        }
        dp[k_len][p_len].iter().fold(0, |acc, &x| (acc + x) % k_mod)
    }
}