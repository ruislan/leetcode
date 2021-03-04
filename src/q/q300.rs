use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // 方法1
        // 动态规划之记忆化搜索（深度+剪枝）
        // 我们分析数组可以做出backtrace的方法
        // 然后我们发现每次backtrace都会有相同的搜索
        // 所以我们需要存储已经搜索过的路径
        // 这样，当我们再次搜索到这个路径的时候
        // 我们就可以直接取它的值即可
        // AC 108ms 2.2mb
        // fn dfs(arr: &mut Vec<i32>, nums: &Vec<i32>, i: usize, memo: &mut Vec<i32>) -> i32 {
        //     if i == nums.len() { return 0; }
        //     if memo[i] > 0 { return memo[i]; }
        //
        //     let mut total = 0;
        //     for j in i..nums.len() {
        //         if let Some(&top) = arr.last() {
        //             if top < nums[j] {
        //                 arr.push(nums[j]);
        //             } else {
        //                 continue;
        //             }
        //         } else {
        //             arr.push(nums[j]);
        //         }
        //         total = total.max(1 + (dfs(arr, nums, j + 1, memo)));
        //         arr.pop();
        //     }
        //     memo[i] = total;
        //     total
        // }
        // dfs(&mut Vec::new(), &nums, 0, &mut vec![0; nums.len()])

        // 方法2
        // 动态规划之状态转移
        // 状态如何转移的，在于当前这个数字之前的数字比它小的有哪些
        // 找到一个比它小的，然后再累加这个比它小的数字的值 + 1就是当前这个数字的
        // 也就是说迭代就是从0..i，i就是当前这个数字
        // 而dp存储的就是所有数字中的最优解
        // 当然这次全局最优解不在最后，而在某一个最优解上
        // O(n^2)
        // AC 76ms 2.1mb
        // let n = nums.len();
        // let mut dp = vec![1; n];
        // for row in 0..n {
        //     for col in 0..row {
        //         if nums[row] > nums[col] {
        //             dp[row] = dp[row].max(dp[col] + 1);
        //         }
        //     }
        // }
        // dp.into_iter().max().unwrap()

        // 方法3
        // 这个是我没想到的，看了官方题解来写的，后面会来复习一下
        // 贪心+二分查找的思路
        // 贪心思路：
        // 假设我们查找到1个数字，后面的数字肯定是越接近这个数字，
        // 那么我们的递增序列才越长
        // 例如：  10,9,2,5,3,7,101,18
        // 10 101和10 18，我们肯定选择10 18
        // 2 5 7 18 和 2 3 7 18我们肯定选后者，虽然结果都一样
        // 那么如何来保证这个序列是增长呢还是不增长呢？
        // 很简单，就是假设遇到一个更小的数字，那么就替换这个序列中的刚好比它大的数字
        // 如果比最后一个数字都大，那就添加最后一个数字
        // 这个过程很巧妙，可以参考官方解的动图
        // O(nlogn)
        // AC 0ms 2.1mb
        let n = nums.len();
        let mut answer = Vec::new();
        for x in nums {
            if answer.is_empty() || x > answer[answer.len() - 1] {
                answer.push(x);
                continue;
            }
            let i = answer.binary_search(&x).unwrap_or_else(|i| i);
            answer[i] = x;
        }
        answer.len() as i32
    }
}