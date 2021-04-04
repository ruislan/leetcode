use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        // 方法1
        // 用栈来做()的匹配，只有匹配的和字母才保留
        // AC 4ms 2.7mb
        let n = s.len();
        let mut stack = Vec::new();
        let mut keep = vec![false; n];
        for (i, ch) in s.chars().enumerate() {
            if ch == '(' { 
                stack.push(i);
            } else if ch == ')' {
                if !stack.is_empty() {
                    keep[stack.pop().unwrap()] = true;
                    keep[i] = true;
                }
            } else {
                keep[i] = true;
            }
        }
        let mut answer = Vec::new();
        for (i, ch) in s.chars().enumerate() {
            if keep[i] {
                answer.push(ch);
            }
        }
        answer.into_iter().collect()
    }
}