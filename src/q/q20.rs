use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch == '(' || ch == '[' || ch == '{' {
                stack.push(ch);
            } else if ch == ')' {
                if stack.pop() != Some('(') { return false; }
            } else if ch == ']' {
                if stack.pop() != Some('[') { return false; }
            } else if ch == '}' {
                if stack.pop() != Some('{') { return false; }
            }
        }
        stack.is_empty()
    }
}