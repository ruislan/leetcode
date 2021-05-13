use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        // 方法1
        // 我们每一个step都有三个选择，不动，向左，向右，
        // 当然也要考虑到在左右边界会少掉向左或者向右
        // 这样一看就是个典型的寻路问题了，最后我们要回到起点，也就是索引0处
        // 那么通过深度优先，我们可以暴力把所有的结果都探到，这样有多少成功的路就是结果
        // 但是这样必然会TLE，所以我们要加上记忆化，
        // 记忆化我们只需要记录当前位置和当前的步数即可
        // 因为假设第2步到索引3这个位置我们记忆过，那么后续的路我们必然走过了，所以就不用走了
        // AC 100ms 14.5mb
        use std::collections::HashMap;
        fn dfs(pos: usize, steps: i32, arr_len: usize, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
            if steps == 0 {
                if pos == 0 { return 1; }
                return 0;
            }

            if let Some(&sum) = memo.get(&(pos, steps)) {
                return sum;
            }
            let mut sum = dfs(pos, steps - 1, arr_len, memo);
            if pos < arr_len - 1 {
                sum += dfs(pos + 1, steps - 1, arr_len, memo);
                sum = sum % 1000000007;
            }
            if pos > 0 {
                sum += dfs(pos - 1, steps - 1, arr_len, memo);
                sum = sum % 1000000007;
            }
            memo.insert((pos, steps), sum);
            sum
        }

        dfs(0, steps, arr_len as usize, &mut HashMap::new())

        // 方法2
        // 动态规划
        // 其实从dfs+记忆化是比较容易找到状态转移方程的
        // 这里当前这一步的累计就是前一步的前面左边，中间和右边累计而来，当我们走完所有的步骤之后
        // 查看第0索引位置的步数累计即可
        // dp[steps][indices]
        // 答案就在dp[steps][0]
        // 后面如果再出现每日一题再来做吧
    }
}