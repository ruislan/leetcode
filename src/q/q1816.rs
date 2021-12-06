use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        // 方法1
        // 这个没啥好说的，就是根据空格分割然后取前k个就行了
        // AC 0ms 2.1mb 72/72
        s.split_ascii_whitespace()
            .enumerate()
            .filter(|(i, w)| *i < k as usize)
            .map(|(i, w)| w)
            .collect::<Vec<_>>()
            .join(" ")
    }
}