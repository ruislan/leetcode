use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.is_empty() && b.is_empty() { return true; }
        if a.len() != b.len() { return false; }

        let mut start = 0;
        let a_chars = a.chars().collect::<Vec<char>>();
        let b_chars = b.chars().collect::<Vec<char>>();
        while start < a.len() {
            let mut p = start;
            let mut good = true;
            for i in 0..b.len() {
                if a_chars[p] != b_chars[i] {
                    good = false;
                    break;
                }
                p += 1;
                if p == a.len() { p = 0; }
            }
            if good { return true; }
            start += 1;
        }
        false
    }
}