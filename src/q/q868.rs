use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut ones = Vec::new();
        let mut i = 0;
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 { ones.push(i); }
            n /= 2;
            i += 1;
        }
        let mut max = 0;
        for j in 1..ones.len() {
            if ones[j] - ones[j - 1] > max {
                max = ones[j] - ones[j - 1];
            }
        }
        max
    }
}