use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        // 方法1
        // 回溯法直接干就完了
        // AC 52ms 2mb
        fn dfs(i: i32, n: i32, used: &mut Vec<bool>) -> i32 {
            if i > n { return 1; }
            let mut sum = 0;
            for j in 1..=n {
                if !used[j as usize] && (j % i == 0 || i % j == 0) {
                    used[j as usize] = true;
                    sum += dfs(i + 1, n, used);
                    used[j as usize] = false;
                }
            }
            sum
        }
        dfs(1, n, &mut vec![false; n as usize + 1])
    }
}
