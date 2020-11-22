use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reformat(s: String) -> String {
        // 方法1
        // 将字符串s的每个字符ch按照数字和字母映射到两个不同的数组nums和chars中
        // 如果nums和chars的len相差的绝对值超过1，则返回空串
        // 然后大的那个数组开始排列，逐个插入成字符串返回
        // Passed 0ms 2mb
        let (nums, chars): (Vec<char>, Vec<char>) = s.chars().partition(|&ch| ch.is_ascii_digit());
        if (nums.len() as i32 - chars.len() as i32).abs() > 1 { return String::new(); }
        let (mut long, mut short) = (chars.iter(), nums.iter());
        if chars.len() < nums.len() {
            long = nums.iter();
            short = chars.iter();
        }
        let (mut res, len) = (String::new(), nums.len() + chars.len());
        while res.len() != len {
            if let Some(&a) = long.next() { res.push(a); }
            if let Some(&b) = short.next() { res.push(b); }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reformat("a0b1c2".to_string()), "a0b1c2".to_string());
    assert_eq!(Solution::reformat("leetcode".to_string()), "".to_string());
    assert_eq!(Solution::reformat("1229857369".to_string()), "".to_string());
    assert_eq!(Solution::reformat("covid2019".to_string()), "c2o0v1i9d".to_string());
    assert_eq!(Solution::reformat("ab123".to_string()), "1a2b3".to_string());
    assert_eq!(Solution::reformat("a1".to_string()), "a1".to_string());
    assert_eq!(Solution::reformat("a11".to_string()), "1a1".to_string());
    assert_eq!(Solution::reformat("1".to_string()), "1".to_string());
    assert_eq!(Solution::reformat("a".to_string()), "a".to_string());
}