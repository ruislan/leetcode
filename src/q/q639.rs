use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_decodings_2(s: String) -> i32 {
        // 方法1
        // 动态规划，这个是91题的进阶版本
        // AC 4ms 2.8mb
        fn ways1(c: char) -> i64 {
            if c == '0' { 0 } else if c == '*' { 9 } else { 1 }
        }
        fn ways2(c1: char, c2: char) -> i64 {
            if c1 == '*' && c2 == '*' {
                15
            } else if c1 == '*' {
                if c2 >= '0' && c2 <= '6' { 2 } else { 1 }
            } else if c2 == '*' {
                if c1 == '1' { 9 } else if c1 == '2' { 6 } else { 0 }
            } else {
                let prefix = (c1 as i64 - '0' as i64) * 10 + (c2 as i64 - '0' as i64);
                if prefix >= 10 && prefix <= 26 { 1 } else { 0 }
            }
        }
        if s.is_empty() { return 0; }
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![1_i64, ways1(s[0])];
        for i in 1..s.len() {
            let mut dp_i = ways1(s[i]) * dp[1] + ways2(s[i - 1], s[i]) * dp[0];
            dp_i %= 1000000007;
            dp[0] = dp[1];
            dp[1] = dp_i;
        }
        dp[1] as i32
    }
}