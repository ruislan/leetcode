use crate::offer::Solution;
use std::borrow::BorrowMut;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 方法1
        // 利用库函数
        // 直接通过split whitespace分割字符串存储成vec
        // 然后翻转vec之后，再组合成String返回
        // Passed 0ms 2.1mb
        s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")

        // 方法2
        // 一次成型，创建一个res为String和空格计数gap
        // s.chars()，如果不是空格，空格计数大于0，就res.push(空格)和res.push(ch)，
        // 并将空格计数计入空，如果是空格，就不Push，将空格计数加一gap += 1
        // 返回res
        // Passed 0ms 2.2mb
        // let mut res = Vec::new();
        // let mut str = String::new();
        // for ch in s.chars() {
        //     match ch {
        //         ' ' => {
        //             if !str.is_empty() { res.push(str); }
        //             str = String::new();
        //         }
        //         _ => str.push(ch),
        //     }
        // }
        // if !str.is_empty() { res.push(str); }
        // res.reverse();
        // res.join(" ")
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_words("".to_string()), "".to_string());
    assert_eq!(Solution::reverse_words("a".to_string()), "a".to_string());
    assert_eq!(Solution::reverse_words("  a some dt".to_string()), "dt some a".to_string());
    assert_eq!(Solution::reverse_words("the sky is blue".to_string()), "blue is sky the".to_string());
    assert_eq!(Solution::reverse_words("  hello world!  ".to_string()), "world! hello".to_string());
    assert_eq!(Solution::reverse_words("a good   example".to_string()), "example good a".to_string());
}