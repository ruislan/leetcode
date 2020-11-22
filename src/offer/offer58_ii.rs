use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        // 方法1
        // 然后取a = &[..n]和b = &[n..]
        // 然后将b放到前面加上a再返回String
        // let mut res = String::from(&s[n as usize..]);
        // res.push_str(&s[..n as usize]);
        // res
        String::from(&s[n as usize..]) + &s[..n as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_left_words("".to_string(), 0), "".to_string());
    assert_eq!(Solution::reverse_left_words("a".to_string(), 1), "a".to_string());
    assert_eq!(Solution::reverse_left_words("ab".to_string(), 1), "ba".to_string());
    assert_eq!(Solution::reverse_left_words("abcdefg".to_string(), 2), "cdefgab".to_string());
    assert_eq!(Solution::reverse_left_words("abcdefg".to_string(), 6), "gabcdef".to_string());
    assert_eq!(Solution::reverse_left_words("lrloseumgh".to_string(), 6), "umghlrlose".to_string());
}