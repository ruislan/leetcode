use crate::offer::Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        // 方法1
        // 将n与s.len()取余数
        // 然后取a = &[..n]和b = &[n..]
        // 然后将b放到前面加上a再返回String
        String::new()
    }
}