use crate::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        // 方法1
        // 元音有a,e,i,o,u
        // 迭代字符串，如果是元音，提取出来，放入栈
        // 迭代字符串，如果是元音，栈pop一个，替换当前这个（代码可以创建一个空串，然后依次push字符，元音的push栈pop的）

        // 方法2
        // 双指针，左边找到元音，右边找到元音，swap，直到le < re
        "".to_string()
    }
}