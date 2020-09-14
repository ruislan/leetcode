use crate::q::Solution;

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n / 5;
            n /= 5;
        }
        res
    }
}