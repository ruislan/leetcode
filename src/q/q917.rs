use crate::q::Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s.len() <= 1 { return s; }

        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            let is_left_alphabet = chars[left].is_ascii_alphabetic();
            let is_right_alphabet = chars[right].is_ascii_alphabetic();

            if is_left_alphabet && !is_right_alphabet {
                right -= 1;
            } else if !is_left_alphabet && is_right_alphabet {
                left += 1;
            } else {
                if is_left_alphabet && is_right_alphabet {
                    chars.swap(left, right);
                }
                left += 1;
                right -= 1;
            }
        }
        chars.iter().collect::<String>()
    }
}