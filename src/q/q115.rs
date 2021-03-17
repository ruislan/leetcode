use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        // 方法1
        // 回溯算法
        // 这里由于数据量较大，肯定会超时
        // 所以需要记忆化或者剪枝，后面来处理吧，我们先换成dp来处理
        // 超时
        // use std::collections::HashMap;
        // fn dfs(path: &mut Vec<usize>, t_i: usize, t: &Vec<u8>, freq: &HashMap<u8, Vec<usize>>, memo: &mut HashMap<usize, i32>) -> i32 {
        //     if path.len() == t.len() { return 1; }
        //     let mut count = 0;
        //     let selection = freq.get(&t[t_i]);
        //     if selection.is_none() { return 0; }
        //     let selection = selection.unwrap();
        //     for i in 0..selection.len() {
        //         if path.is_empty() || selection[i] > path[path.len() - 1] {
        //             path.push(selection[i]);
        //             count += dfs(path, t_i + 1, t, freq, memo);
        //             path.pop();
        //         }
        //     }
        //     count
        // }
        //
        // let mut freq = HashMap::new();
        // for (i, b) in s.into_bytes().into_iter().enumerate() {
        //     freq.entry(b).or_insert(Vec::new()).push(i);
        // }
        //
        // dfs(&mut Vec::new(), 0, &t.into_bytes(), &freq, &mut HashMap::new())

        // 方法2
        // 动态规划
        // 还是先找dp[row][col]代表什么，通常这个位置代表的都是子问题的最优解
        // 接下来我们来看Rows和cols是什么，这里很明显就是两个字符串了
        // 然后是状态转移，通常我们状态转移分为叠加状态和转移状态，
        // 其实这个题很好找两个状态，两个字符串无外乎就是s[i] 与 t[j]是否相等
        // 如果相等，说明有字符匹配了，那么这个匹配的字符自然需要叠加之前匹配的字符
        // 那么之前的字符在dp[row - 1][col - 1]和dp[row - 1][col]中，我们把这两个加起来
        // 如果不相等，说明s[i] != t[j]，那么只能沿用之前的状态，也就是dp[row - 1][col]
        // 最后结果就在最右下角的格子中
        // 例如： babgbag 与 bag
        //      b   a   g
        //    1 0   0   0  这一行表示空字符的时候，和空字符是相等的，但是其他的是不相等的，作为初始状态
        // b  1 1   0   0  这里b相等的时候，状态叠加过来，也就是左上角和上方的值的和，不相等就直接转移上面的值
        // a  1 1   1   0
        // b  1 2   1   0
        // g  1 2   1   1
        // b  1 3   1   1
        // a  1 3   4   1
        // g  1 3   4   5
        // 下面的代码，我们由于多加了一行和一列，所以我们实际上dp[row][col]都变成了dp[row+1][col+1]
        // AC 0ms 2.3mb
        let rows = s.len() + 1;
        let cols = t.len() + 1;
        let mut dp = vec![vec![0; cols]; rows];
        for i in 0..rows { dp[i][0] = 1; }
        for (row, s) in s.chars().enumerate() {
            for (col, t) in t.chars().enumerate() {
                dp[row + 1][col + 1] = if s == t {
                    dp[row][col] + dp[row][col + 1]
                } else {
                    dp[row][col + 1]
                };
            }
        }
        dp[rows - 1][cols - 1]
    }
}