use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        // 方法1
        // Passed 44ms 2.2mb
        // let mut set = std::collections::HashSet::new();
        // let mut words = words;
        // words.sort_by(|a, b| b.len().cmp(&a.len()));
        // for word in &words {
        //     set.insert(word);
        // }
        // let mut max_words = Vec::new();
        // let mut last_len = 0;
        // for word in &words {
        //     if word.len() < last_len { break; }
        //     let mut chars = word.chars().collect::<Vec<char>>();
        //     while !chars.is_empty() {
        //         let cur_word = chars.iter().collect::<String>();
        //         if !set.contains(&cur_word) { break; }
        //         chars.pop();
        //     }
        //     if chars.is_empty() {
        //         last_len = word.len();
        //         max_words.push(word.to_string());
        //     }
        // }
        // max_words.sort();
        // max_words[0].to_string()

        // 方法2
        // Passed 8ms 2.3mb
        let mut set = std::collections::HashSet::new();
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut res = String::new();
        for word in words {
            if word.len() <= 1 || set.contains(&word[..(word.len() - 1)].to_string()) {
                set.insert(word.clone());
                if word.len() > res.len() || (word.len() == res.len() && &word < &res) {
                    res = word;
                }
            };
        }
        res
    }
}