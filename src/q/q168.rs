use crate::q::Solution;

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut res = String::new();
        let mut n = n;
        while n > 0 {
            let ch = n % 26;
            n = n / 26;
            match ch {
                0 => {
                    n -= 1;
                    res.insert(0, 'Z');
                }
                _ => res.insert(0, (ch as u8 + 64) as char),
            }
        }
        res
    }
}