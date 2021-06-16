use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 方法1
        // 直接字符串分割，然后反转
        // AC 0ms 2.2mb
        let mut answer = String::new();
        s.split_ascii_whitespace().rev().for_each(|x| {
            answer.push_str(x);
            answer.push(' ');
        });
        answer.pop();
        answer

        // 方法2
        // 直接反转s，然后再逐个反转每个单词
        // 今天没时间了，下次遇到这个每日一题来用这个方式做一次
    }
}