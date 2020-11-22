use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // 方法1
        // let mut count = 0i32;
        // loop {
        //     let ch = s.pop();
        //     if None == ch { break; }
        //     if ch == Some(' ') && count == 0 { continue; }
        //     if ch == Some(' ') { break; }
        //     count += 1;
        // }
        // count

        // 方法2
        let v: Vec<&str> = s.trim().rsplit(' ').collect();
        if v.is_empty() { 0i32 } else { v[0].len() as i32 }
    }
}