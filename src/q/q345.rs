use crate::q::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        // 方法1
        // 元音有a,e,i,o,u
        // 迭代字符串，如果是元音，提取出来，放入栈
        // 迭代字符串，如果是元音，栈pop一个，替换当前这个（代码可以创建一个空串，然后依次push字符，元音的push栈pop的）
        // Passed 4ms 2.3mb
        // let mut stack: Vec<char> = s.chars().filter(|x| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(x)).collect();
        // s.chars().map(|x| if ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&x) { stack.pop().unwrap_or(x) } else { x }).collect()

        // 方法2
        // 双指针，左边找到元音，右边找到元音，swap，直到le < re
        // Passed 0ms 2.7mb
        if s.is_empty() { return s; }
        let (mut le, mut re) = (0, s.len() - 1);
        let mut chars: Vec<char> = s.chars().collect();
        while le < re {
            while !['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&chars[le]) && le < re { le += 1; }
            while !['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&chars[re]) && le < re { re -= 1; }
            if le < re {
                chars.swap(le, re);
                le += 1;
                re -= 1;
            }
        }
        chars.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_vowels("".to_string()), "".to_string());
    assert_eq!(Solution::reverse_vowels("a".to_string()), "a".to_string());
    assert_eq!(Solution::reverse_vowels("ae".to_string()), "ea".to_string());
    assert_eq!(Solution::reverse_vowels("ffcc".to_string()), "ffcc".to_string());
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    assert_eq!(Solution::reverse_vowels("leetcOde".to_string()), "leOtcede".to_string());
}