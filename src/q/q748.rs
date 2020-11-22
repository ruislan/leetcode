use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut bucket = vec![0; 26];
        for ch in license_plate.chars() {
            if ch.is_ascii_alphabetic() {
                let idx = ch.to_ascii_lowercase() as u8 - 'a' as u8;
                bucket[idx as usize] += 1;
            }
        }

        let mut res = String::new();

        for i in 0..words.len() {
            let mut v = vec![0; 26];
            let word = &words[i];
            for ch in word.chars() {
                let idx = ch.to_ascii_lowercase() as u8 - 'a' as u8;
                v[idx as usize] += 1;
            }
            let mut found = true;
            for j in 0..v.len() {
                if bucket[j] > v[j] {
                    found = false;
                    break;
                }
            }
            if found && (res.is_empty() || words[i].len() < res.len()) {
                res = words[i].clone();
            }
        }

        res
    }
}