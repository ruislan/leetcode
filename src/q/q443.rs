use crate::q::Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut ptr = 0;
        let mut write = 0;
        for i in 0..chars.len() {
            if i + 1 == chars.len() || chars[i + 1] != chars[i] {
                chars[write] = chars[ptr];
                write += 1;
                if i > ptr {
                    for c in (i - ptr + 1).to_string().chars() {
                        chars[write] = c;
                        write += 1;
                    }
                }
                ptr = i + 1;
            }
        }
        write as i32
    }
}