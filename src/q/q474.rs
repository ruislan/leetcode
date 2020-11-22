use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for str in strs {
            let mut count = (0, 0);
            for ch in str.chars() {
                match ch {
                    '0' => count.0 += 1,
                    '1' => count.1 += 1,
                    _ => (),
                }
            }
            for zeros in (count.0..=m).rev() {
                for ones in (count.1..=n).rev() {
                    dp[zeros][ones] =
                        std::cmp::max(1 + dp[zeros - count.0][ones - count.1], dp[zeros][ones]);
                }
            }
        }
        dp[m][n]
    }
}