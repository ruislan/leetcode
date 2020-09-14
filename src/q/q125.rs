use crate::q::Solution;

impl Solution {
    // 方法名is_palindrome与q9相同，重命名此方法is_palindrome_string
    pub fn is_palindrome_string(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for mut ch in s.chars() {
            if ch.is_ascii_digit() || ch.is_ascii_alphabetic() {
                ch.make_ascii_lowercase();
                stack.push(ch);
            }
        }
        let len = stack.len();
        for i in 0..len / 2 {
            if stack[i] != stack[len - 1 - i] { return false; }
        }
        true
    }
}