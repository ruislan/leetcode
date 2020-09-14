use crate::q::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // 方法1
        // s.reverse()

        // 方法2
        if s.len() <= 1 { return; }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}