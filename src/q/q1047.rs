use crate::q::Solution;

impl Solution {
    // 方法名remove_duplicates与q26相同，更名为remove_duplicates_string
    pub fn remove_duplicates_string(s: String) -> String {
        let mut stack = Vec::new();
        for ch in s.chars() {
            if stack.is_empty() {
                stack.push(ch);
            } else {
                if let Some(&x) = stack.last() {
                    if x == ch { stack.pop(); } else { stack.push(ch); }
                }
            }
        }
        stack.iter().collect()
    }
}