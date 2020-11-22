use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // 方法1
        // let tb: Vec<u8> = t.bytes().collect();
        // let mut p = 0;
        // let mut count = 0;
        // for sb in s.bytes() {
        //     for i in p..tb.len() {
        //         if sb == tb[i] {
        //             p = i + 1;
        //             count += 1;
        //             break;
        //         }
        //     }
        // }
        // count == s.len()

        // 方法2
        // let mut count = 0;
        // let mut sb = s.as_bytes();
        // for b in t.bytes() {
        //     if count == s.len() { break; }
        //     if sb[count] == b {
        //         count += 1;
        //     }
        // }
        // count == sb.len()

        // 方法3
        fn is_sub(s: &str, t: &str) -> bool {
            if s.len() == 0 { return true; }
            if t.len() == 0 { return false; }
            let first_char = s.chars().next().unwrap();
            let found = t.char_indices().find(|&x| x.1 == first_char);
            return if found == None { false } else {
                is_sub(
                    &s[1..],
                    &t[found.unwrap().0 + 1..],
                )
            };
        }
        is_sub(&s, &t)
    }
}