use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 方法1
        // 利用库函数
        // 直接通过split whitespace分割字符串存储成vec
        // 然后翻转vec之后，再组合成String返回
        // Passed 0ms 2.1mb
        s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")

        // 方法2
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