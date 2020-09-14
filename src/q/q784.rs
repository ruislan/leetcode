use crate::q::Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut vec = vec![String::new(); 1];
        for ch in s.chars() {
            if !ch.is_ascii_alphabetic() {
                for i in 0..vec.len() {
                    vec[i].push(ch);
                }
            } else {
                let uppercase = ch.to_ascii_uppercase();
                let lowercase = ch.to_ascii_lowercase();

                for i in 0..vec.len() {
                    let mut new_s = vec[i].clone();
                    new_s.push(uppercase);
                    vec.push(new_s);

                    vec[i].push(lowercase);
                }
            }
        }
        vec
    }
}