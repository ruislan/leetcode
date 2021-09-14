use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        // 方法1
        // 字典按照长度排序，长度相等，按照自然顺序排序
        // 依次检查是否符合字符，
        // 如果全部都检查后没有，则返回空字符串
        // AC 12ms 3mb
        use std::cmp::Ordering;
        let mut s: Vec<char> = s.chars().collect();
        let mut dic = dictionary;
        dic.sort_unstable_by(|a, b| {
            let cmp = b.len().cmp(&a.len());
            if cmp == Ordering::Equal {
                a.cmp(b)
            } else {
                cmp
            }
        });
        for d in dic {
            let mut chars: Vec<char> = d.chars().collect();
            let mut i = 0;
            let mut j = 0;
            while i < chars.len() && j < s.len() {
                if chars[i] == s[j] { i += 1; }
                j += 1;
            }
            if i == chars.len() { return d; }
        }
        String::from("")
    }
}