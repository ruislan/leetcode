use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // 方法1
        // 利用双map对pattern和s进行对应
        // 如果两个都没有，那么放入maps
        // 如果其中一个有，那么返回false
        // 如果两个都有，那么检查彼此存储的是否相同
        // Passed 0ms 2mb
        let mut pattern_map = std::collections::HashMap::new();
        let mut word_map = std::collections::HashMap::new();
        let words: Vec<&str> = s.split(' ').collect();
        let patterns: Vec<char> = pattern.chars().collect();

        if patterns.len() != words.len() { return false; }
        for i in 0..words.len() {
            let pattern = *word_map.entry(words[i]).or_insert(patterns[i]);
            let word = *pattern_map.entry(patterns[i]).or_insert(words[i]);
            if pattern != patterns[i] || word != words[i] { return false; }
        }
        true
    }
}