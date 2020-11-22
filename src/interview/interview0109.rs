use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        // 方法1
        // 首先检查两个字符串长度是否相等，不相等，直接返回false
        // 然后从索引0开始到索引len - 1分割字符串分别为,a和b
        // 然后组合b和a，与s2比较，如果相等则返回true
        // 迭代结束返回false
        // if s1.len() != s2.len() { return false; }
        // for i in 0..s1.len() {
        //     if s1[i..].to_string() + &s1[..i] == s2 { return true; }
        // }
        // s1.is_empty()

        // 方法2
        // 如果s2由s1旋转而成，那么s2一定在s1 + s1中
        s1.len() == s2.len() && s1.repeat(2).contains(&s2)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_fliped_string("".to_string(), "".to_string()), true);
    assert_eq!(Solution::is_fliped_string("a".to_string(), "a".to_string()), true);
    assert_eq!(Solution::is_fliped_string("ab".to_string(), "ba".to_string()), true);
    assert_eq!(Solution::is_fliped_string("ab".to_string(), "ab".to_string()), true);
    assert_eq!(Solution::is_fliped_string("aa".to_string(), "ab".to_string()), false);
    assert_eq!(Solution::is_fliped_string("waterbottle".to_string(), "erbottlewat".to_string()), true);
}