use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        // 方法1
        // let mut bag = vec![0;26];
        // for b in chars.bytes() {
        //     bag[b as usize - 97] += 1;
        // }
        // let mut count = 0;
        // 'outer:for word in words {
        //     let mut dic = vec![0;26];
        //     for w in word.bytes() {
        //         dic[w as usize - 97] += 1;
        //         if dic[w as usize - 97] > bag[w as usize - 97] {
        //             continue 'outer;
        //         }
        //     }
        //     count += word.len() as i32;
        // }
        // count
        
        // 方法2
        let mut bag = vec![0; 26];
        for b in chars.bytes() {
            bag[b as usize - 97] += 1;
        }
        let mut count = 0;
        for word in words {
            let mut dic = vec![0; 26];
            let mut hit = true;
            for w in word.bytes() {
                dic[w as usize - 97] += 1;
                if dic[w as usize - 97] > bag[w as usize - 97] {
                    hit = false;
                    break;
                }
            }
            if hit { count += word.len() as i32; }
        }
        count
    }
}