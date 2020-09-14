use crate::q::Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        // 方法1：倒序迭代s，遇到'#'，则取#前两个，得到字母，没有则直接得到映射字母
        // Passed 0ms 2mb
        // let mut chars = s.chars().collect::<Vec<char>>();
        // let mut res = Vec::new();
        // let mut ptr = chars.len() as i32 - 1;
        // while ptr >= 0 {
        //     let mut s = String::new();
        //     if '#' == chars[ptr as usize] {
        //         s.push(chars[ptr as usize - 2]);
        //         s.push(chars[ptr as usize - 1]);
        //         ptr -= 3;
        //     } else {
        //         s.push(chars[ptr as usize]);
        //         ptr -= 1;
        //     }
        //
        //     if let Ok(num) = s.parse::<u8>() {
        //         res.push((96 + num) as char);
        //     }
        // }
        //
        // res.reverse();
        // res.iter().collect()
        // graceful 方案
        // let mut res = Vec::new();
        // let mut ptr = s.len() as i32 - 1;
        // while ptr >= 0 {
        //     let i = ptr as usize;
        //     let mut range = i..=i;
        //     if "#" == &s[i..=i] {
        //         range = (i - 2)..=(i - 1);
        //         ptr -= 2;
        //     }
        //     res.push((&s[range].parse::<u8>().unwrap_or(0) + 96) as char);
        //     ptr -= 1;
        // }
        // res.reverse();
        // res.iter().collect()

        // 方法二，向前迭代，找到i + 2，如果是#，则取i..=i+1，然后i += 2
        // Passed 0ms 2mb
        let mut res = String::new();
        let mut ptr = 0;
        while ptr < s.len() {
            let mut range = ptr..=ptr;
            if ptr + 2 < s.len() && "#" == &s[(ptr + 2)..=(ptr + 2)] {
                range = ptr..=(ptr + 1);
                ptr += 2;
            }
            res.push((&s[range].parse::<u8>().unwrap_or(0) + 96) as char);
            ptr += 1;
        }
        res
    }
}

#[test]
fn test_q1309() {
    assert_eq!(&Solution::freq_alphabets("12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()), "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(&Solution::freq_alphabets("25#".to_string()), "y");
    assert_eq!(&Solution::freq_alphabets("1326#".to_string()), "acz");
    assert_eq!(&Solution::freq_alphabets("10#11#12".to_string()), "jkab");
}