use crate::q::Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let len = word.len();
        let mut upper = 0;
        let mut first_upper = false;
        let chars = word.chars().collect::<Vec<char>>();
        for i in 0..len {
            let ch = chars[i];

            if ch.is_ascii_uppercase() {
                if i == 0 { first_upper = true; }
                upper += 1;
            }
        }
        if upper == 0 { return true; }
        if upper == len { return true; }
        if len > 1 && upper == 1 && first_upper { return true; }
        false
    }
}