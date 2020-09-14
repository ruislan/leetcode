use crate::q::Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut cur = 0;
        let mut line = 1;
        for ch in s.chars() {
            let width = widths[(ch as u8 - 'a' as u8) as usize];
            cur += width;
            if cur > 100 {
                line += 1;
                cur = width;
            }
        }
        vec![line, cur]
    }
}