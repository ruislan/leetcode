use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 回溯搜索
        // 其实就是暴力的列出所有的可能性，这里注意每次其实只有+，-两个选择
        // AC 492ms 2.2mb
        // fn dfs(mut sum: i32, i: usize, nums: &Vec<i32>, target: i32, answer: &mut i32) {
        //     if i == nums.len() {
        //         if sum == target { *answer += 1; }
        //         return;
        //     }
        //     dfs(sum + nums[i], i + 1, nums, target, answer);
        //     dfs(sum - nums[i], i + 1, nums, target, answer);
        // }
        // let mut answer = 0;
        // dfs(0, 0, &nums, target, &mut answer);
        // answer

        // 方法2
        // 动态规划
        // dp[i][j]肯定存储的就是局部最优解了，那么就是最多的解法
        // 而i和j分别代表什么，这里其实还是比较容易得出来，j表示的是当前的和sum
        // 而i表示的是当前的数字nums[i]，那么sum其实是从-x,x这个范围，例如[1,1,1,1,1]就是-5,5
        // 我们列出来：
        //        -5   -4   -3   -2   -1   0   1   2   3   4   5
        //   0                             1
        //   1                        1        1
        //   2                    1       2        1
        //   3              1         3        3       1
        //   4          1         4       6        4       1
        //   5     1        5         10      10      5        1
        //  然后我们的答案就在dp[5][target]中，这里就是5
        // AC 4ms 2.1mb
        let mut sum = nums.iter().sum::<i32>();
        if sum < target { return 0; }
        let sum = sum as usize;
        let rows = nums.len();
        let cols = sum * 2 + 1;
        let mut dp = vec![vec![0; cols]; rows + 1];
        dp[0][sum] = 1;
        for i in 0..rows {
            for j in nums[i] as usize..cols - nums[i] as usize {
                if dp[i][j] > 0 {
                    dp[i + 1][j + nums[i] as usize] += dp[i][j];
                    dp[i + 1][j - nums[i] as usize] += dp[i][j];
                }
            }
        }
        dp[rows][sum + target as usize]
    }
}