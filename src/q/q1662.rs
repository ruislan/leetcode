use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        // 方法1
        // 将两个vec的字符串起来，比较即可
        // Passed 0ms 2.3mb
        let (mut s1, mut s2) = (String::new(), String::new());
        word1.into_iter().for_each(|x| s1.push_str(&x));
        word2.into_iter().for_each(|x| s2.push_str(&x));
        s1 == s2
    }
}