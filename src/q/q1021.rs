use crate::q::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        // 方法1
        // Passed 96ms 2.2mb
        // let mut stack = Vec::new();
        // let mut chars: Vec<char> = s.chars().collect();
        // let mut idx = Vec::new();
        // for i in 0..chars.len() {
        //     if '(' == chars[i] { stack.push(i); } else if ')' == chars[i] {
        //         if let Some(x) = stack.pop() {
        //             if stack.is_empty() {
        //                 idx.push(x);
        //                 idx.push(i);
        //             }
        //         }
        //     }
        // }
        // let mut res = String::new();
        // for i in 0..chars.len() {
        //     if !idx.contains(&i) {
        //         res.push(chars[i]);
        //     }
        // }
        // res
        // 方法2
        let mut counter = 0;
        let mut result = String::new();
        for ch in s.chars() {
            if ch == ')' {
                counter += 1;
            }
            if counter != 0 {
                result.push(ch);
            }
            if ch == '(' {
                counter -= 1;
            }
        }
        result
    }
}