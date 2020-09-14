use crate::q::Solution;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut ch_map = std::collections::HashMap::<char, &str>::new();
        let mut word_map = std::collections::HashMap::<&str, char>::new();
        let str_v: Vec<&str> = str.split(' ').collect();
        let p_chars: Vec<char> = pattern.chars().collect();
        let len1 = p_chars.len();
        let len2 = str_v.len();
        if len1 != len2 { return false; }
        for i in 0..len1 {
            let p_ch = p_chars[i];
            let word = str_v[i];
            let in_ch_map = ch_map.get(&p_ch);
            let in_word_map = word_map.get(&word);
            if in_ch_map == None && in_word_map != None {
                return false;
            }
            if in_ch_map != None && in_word_map == None {
                return false;
            }
            if in_ch_map != None && in_word_map != None {
                if in_ch_map.unwrap() != &word || in_word_map.unwrap() != &p_ch {
                    return false;
                }
            }
            if in_ch_map == None && in_word_map == None {
                ch_map.insert(p_ch, word);
                word_map.insert(word, p_ch);
            }
        }
        true
    }
}