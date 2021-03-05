use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 回溯算法搞定组合排列全家桶
        // 基本和它前几个系列一样，没啥新东西
        // 简单就是上模板，注意一下出口和进入结果集的条件
        // 一提交还真有新东西，原来是数据量会给你很大
        // 那么就加上记忆化好了，这一下就成了动态规划了
        // AC 0ms 2.6mb
        // use std::collections::HashMap;
        // fn backtrace(path: &mut Vec<i32>, nums: &Vec<i32>, sum: i32, target: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        //     if sum == target { return 1; }
        //     if let Some(cache) = memo.get(&sum) {
        //         return *cache;
        //     }
        //     let mut total = 0;
        //     for i in 0..nums.len() {
        //         if nums[i] + sum > target { break; }
        //         path.push(nums[i]);
        //         total += backtrace(path, nums, nums[i] + sum, target, memo);
        //         path.pop();
        //     }
        //     *memo.entry(sum).or_insert(0) = total;
        //     total
        // }
        // let mut nums = nums;
        // nums.sort_unstable();
        // backtrace(&mut Vec::new(), &nums, 0, target, &mut HashMap::new())

        // 方法2
        // 那既然变成动态规划了，那么我们把深度+记忆化转换成一种广度的形式
        // 也就是说
        //      1       2          3
        //  1   1 2 3   1 2 3      ...
        //  2   ...     ...
        //  3   ...
        //  从每次都看sum = 1,2,3,4的形式，变成了，先看sum=1有多少，再看sum=2有多少，依次累加的形式
        // AC 0ms 2mb
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for j in 0..nums.len() {
                let x = nums[j] as usize;
                if i >= x {
                    dp[i] += dp[i - x];
                }
            }
        }
        dp[target]
    }
}