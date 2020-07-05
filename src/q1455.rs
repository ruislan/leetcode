use crate::Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        // 方法1
        // 用sentence的split_ascii_whitespace方法隔离出每个word的iter
        // 然后iter中find，用word.starts_with(search_word)来判断是否是前缀
        // 找到的话即可返回这个iter的index+1
        // 没有找到的话则返回-1
        -1
    }
}