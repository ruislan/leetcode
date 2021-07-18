use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 方法1
        // 迭代strs，然后strs[i]按照字母排序，放入map中，相同的字符串积累起来
        // AC 12ms 4.8mb
        let mut map = std::collections::HashMap::new();
        for str in strs {
            let mut chars = str.chars().collect::<Vec<char>>();
            chars.sort_unstable();
            let v = map.entry(chars).or_insert(Vec::new());
            v.push(str);
        }
        map.into_iter().map(|(k, v)| v).collect()
    }
}