use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut count_a = 0;
        let mut count_ci_l = 0;
        for ch in s.chars() {
            if ch == 'A' {
                count_a += 1;
                if count_ci_l < 3 { count_ci_l = 0; }
            } else if ch == 'L' && count_ci_l < 3 {
                count_ci_l += 1;
            } else {
                if count_ci_l < 3 { count_ci_l = 0; }
            }
        }

        count_ci_l < 3 && count_a < 2
    }
}