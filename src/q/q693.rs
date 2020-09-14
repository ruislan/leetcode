use crate::q::Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut last_bit = None;
        let mut n = n;
        while n > 0 {
            if None == last_bit {
                last_bit = Some(n % 2);
            } else {
                let cur_bit = Some(n % 2);
                if cur_bit == last_bit { return false; }
                last_bit = cur_bit;
            }
            n /= 2;
        }
        true
    }
}