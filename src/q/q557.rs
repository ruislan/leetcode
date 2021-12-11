use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_words_2(s: String) -> String {
        let s: Vec<&str> = s.split(' ').collect();
        let mut res = String::new();
        for word in s {
            res.push_str(&word.chars().rev().collect::<String>());
            res.push(' ');
        }
        res.pop();
        res
    }
}