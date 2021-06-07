use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        // 方法1
        // 其实可以想象成为，所有的石头都可能会被消除一下，
        // 我们遍历所有的可能，留下最小的但是大于0的那个就是结果
        // TLE O(2^n)
        // fn dfs(sum: i32, i: usize, stones: &Vec<i32>, answer: &mut i32) {
        //     if i == stones.len() {
        //         if sum >= 0 { *answer = sum.min(*answer); }
        //         return;
        //     }
        //     dfs(sum + stones[i], i + 1, stones, answer);
        //     dfs(sum - stones[i], i + 1, stones, answer);
        // }
        // let mut answer = i32::MAX;
        // dfs(0, 0, &stones, &mut answer);
        // answer

        // 方法2
        // 动态规划
        // 其实有了方法1，我们就比较好动态规划了
        // 这个题和q494其实是一个解法了，具体的动态规划可以看q494
        // 唯一的区别就是dp[i][j]表示的就是当前这个和有没有出现过
        // 然后最后我们从结果为0开始走过去，一直走到第一个true的就是结果
        let mut sum = stones.iter().sum::<i32>();
        let sum = sum as usize;
        let rows = stones.len();
        let cols = sum * 2 + 1;
        let mut dp = vec![vec![false; cols]; rows + 1];
        dp[0][sum] = true;
        for i in 0..rows {
            for j in stones[i] as usize..cols - stones[i] as usize {
                if dp[i][j] {
                    dp[i + 1][j + stones[i] as usize] = true;
                    dp[i + 1][j - stones[i] as usize] = true;
                }
            }
        }
        for j in sum..cols {
            if dp[rows][j] { return (j - sum) as i32; }
        }
        0
    }
}

