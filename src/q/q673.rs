use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        // 方法1
        // 动态规划
        // 比较容易找到状态转移方程为 dp[i] = max(dp[i], dp[j] + 1)
        // 也就是第i个位置的数字都是由前面所有比他小的数字中的最长的那个子序列的数字积累而来
        // 然后我们只需要再维护一个数组来记录cnt[i]为当前这个nums[i]最大的数量即可
        // 例如： 1 2 4 3 6 5
        //    dp 1 2 3 3 4 4
        //   cnt 1 1 1 1 2 2
        //   ans 1 1 1 2 2 4
        // 答案就是4
        // AC 16ms 2.1mb 223/223
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut cnt = vec![1; n];
        let mut max = 0;
        let mut ans = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        cnt[i] = cnt[j];
                    } else if dp[j] + 1 == dp[i] {
                        cnt[i] += cnt[j];
                    }
                }
            }
            if dp[i] > max {
                max = dp[i];
                ans = cnt[i];
            } else if dp[i] == max {
                ans += cnt[i];
            }
        }
        ans
    }
}

