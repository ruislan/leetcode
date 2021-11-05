use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        // 方法1
        // 动态规划
        // AC 28ms 3.1mb 39/39
        let mut dp = std::collections::HashMap::new();
        let mut ans = 0;
        for x in arr {            
            let y = dp.get(&(x - difference)).unwrap_or(&0) + 1;
            ans = ans.max(y);
            dp.insert(x, y);
        }
        ans
    }
}
