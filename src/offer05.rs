use crate::Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        // 方法1
        // 利用库函数replace直接替换
        // Passed 0ms 2.1mb
        // s.replace(" ", "%20")

        // 方法2
        // 迭代s中的char，遇到空格就替换成%20
        // Passed 0ms 2.1mb
        s.chars().map(|ch| if !ch.is_ascii_whitespace() { ch.to_string() } else { "%20".to_string() }).collect()
        // 方法2的扩展1
        // Passed 0ms 2mb
        // let mut res = String::new();
        // s.chars().for_each(|ch| if !ch.is_ascii_whitespace() { res.push(ch); } else { res.push_str("%20"); });
        // res

        // 注意：不能切割数组再合并，对于结尾是空格的，会漏算
    }
}

#[test]
fn test() {
    assert_eq!(Solution::replace_space("".to_string()), "".to_string());
    assert_eq!(Solution::replace_space("We".to_string()), "We".to_string());
    assert_eq!(Solution::replace_space("We are ".to_string()), "We%20are%20".to_string());
    assert_eq!(Solution::replace_space("We%20 are ".to_string()), "We%20%20are%20".to_string());
}