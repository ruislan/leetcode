use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // 方法1
        // 这个是一道典型的dp题
        // dp的步骤有三，
        // 第一，确定横纵坐标代表的意义；
        // 第二，确定每个单元格的值代表的意义（一般都是当前最优解，也就是我们要找的答案）；
        // 第三，找出状态转移方程，也就是每个单元格的变化规律；
        // 1.这里我们将两个字符串分别作为横纵坐标
        // 2.然后每个单元格就是当前最大的公共子串的值
        // 3.最后状态转移，
        //   当前横纵坐标背后的字符串相等的时候:
        //       说明我们又找了一个符合的字符，这个时候，我们就将左上方的结果+1作为当前值
        //   否则：
        //       说明我们找到了一个不符合的字符，这个时候
        //       我们需要比较一下我们左边(text1的前一个字符)和上边(text2的前一个字符）他们的值谁大就复制谁
        // 最后返回最右下的结果
        // Passed 4ms 5.9mb
        let mut text1: Vec<char> = text1.chars().collect();
        let mut text2: Vec<char> = text2.chars().collect();
        let rows = text1.len();
        let cols = text2.len();
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                if text1[row - 1] == text2[col - 1] {
                    dp[row][col] = dp[row - 1][col - 1] + 1;
                } else {
                    dp[row][col] = dp[row - 1][col].max(dp[row][col - 1]);
                }
            }
        }
        dp[rows][cols]
    }
}