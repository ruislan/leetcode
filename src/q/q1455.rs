use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        // 方法1
        // 用sentence的split_ascii_whitespace方法隔离出每个word的iter
        // 然后iter中find，用word.starts_with(search_word)来判断是否是前缀
        // 找到的话即可返回这个iter的index+1
        // 没有找到的话则返回-1
        if let Some((i, _)) = sentence.split_ascii_whitespace()
            .enumerate()
            .find(|&(_, word)| word.starts_with(&search_word)) { i as i32 + 1 } else { -1 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_prefix_of_word("i".to_string(), "i".to_string()), 1);
    assert_eq!(Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()), 4);
    assert_eq!(Solution::is_prefix_of_word("this problem is an easy problem".to_string(), "pro".to_string()), 2);
    assert_eq!(Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()), -1);
    assert_eq!(Solution::is_prefix_of_word("i use triple pillow".to_string(), "pill".to_string()), 4);
    assert_eq!(Solution::is_prefix_of_word("hello from the other side".to_string(), "they".to_string()), -1);
}