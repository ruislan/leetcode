use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 方法1
        // if s.len() <= 1 { return s.len() as i32; }
        // let mut max = 0;
        // let chars = s.chars().collect::<Vec<char>>();
        // let len = chars.len();
        // for mut i in 0..len {
        //     let mut temp = String::new();
        //     temp.push(*chars.get(i).unwrap());
        //     for k in (i + 1)..len {
        //         let ch = chars.get(k).unwrap();
        //         if temp.contains(*ch) {
        //             if k - i > max { max = k - i; }
        //             i = k;
        //             break;
        //         } else if k + 1 == len && len - i > max {
        //             max = len - i;
        //             break;
        //         }
        //         temp.push(*ch);
        //     }
        // }
        // max as i32

        // 方法2
        let chars = s.chars().collect::<Vec<char>>();
        let mut map_char = std::collections::HashMap::new();
        let (mut i, mut j, mut ans) = (0, 0, 0);
        while j < s.len() {
            if map_char.contains_key(&chars[j]) {
                i = std::cmp::max(*map_char.get(&chars[j]).unwrap(), i);
            }
            ans = std::cmp::max(ans, j - i + 1);
            map_char.insert(chars[j], j + 1);
            j += 1;
        }
        ans as i32
    }
}