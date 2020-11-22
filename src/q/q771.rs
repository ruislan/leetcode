use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        for ch in j.chars() {
            set.insert(ch);
        }

        let mut sum = 0;
        for ch in s.chars() {
            if set.contains(&ch) { sum += 1; }
        }
        sum
    }
}