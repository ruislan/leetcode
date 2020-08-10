use crate::interview::Solution;

impl Solution {
    pub fn replace_spaces(s: String, length: i32) -> String {
        // 方法1
        // length是实际字符串的长度，将s转换为字符串，并保留length长度
        // 替换其中的空格为%20
        // Passed 52ms 6.6mb
        // s.chars().take(length as usize).map(|ch| if ch.is_ascii_whitespace() { "%20".to_string() } else { ch.to_string() }).collect()

        // 方法2
        // 方法1的优化，字符串在Rust是比较费操作的，我们可以先不做to_string，最后一起来做
        let length = length as usize;
        let mut res = String::new();
        for (i, ch) in s.chars().enumerate() {
            if i == length { break; }
            if ch.is_ascii_whitespace() { res.push_str("%20"); } else { res.push(ch); }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::replace_spaces("".to_string(), 0), "".to_string());
    assert_eq!(Solution::replace_spaces("Mr John Smith    ".to_string(), 13), "Mr%20John%20Smith".to_string());
    assert_eq!(Solution::replace_spaces("               ".to_string(), 5), "%20%20%20%20%20".to_string());
    assert_eq!(Solution::replace_spaces("               ".to_string(), 4), "%20%20%20%20".to_string());
}