use crate::q::Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        // 方法1
        // let len = s.len();
        // if len <= 1 { return false; }
        // let mut i = 1;
        // loop {
        //     let sub = s.get(..i);
        //     if sub.is_none() { return false; }
        //
        //     let rep = len / i;
        //     let mut expect = String::new();
        //     for _ in 0..rep {
        //         expect.push_str(sub.unwrap());
        //     }
        //     if expect == s { return true; }
        //
        //     i += 1;
        //     if i > len / 2 {
        //         return false;
        //     }
        //     while len % i != 0 {
        //         i += 1;
        //         if i > len / 2 {
        //             return false;
        //         }
        //     }
        // }
        // false

        // 方法2
        let ns = s.clone() + s.as_str();
        let sub = ns.get(1..ns.len() - 1);
        if sub.is_none() { false } else { sub.unwrap().contains(&s) }
    }
}