use crate::offer::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 方法1
        // 利用库函数
        // 直接通过split whitespace分割字符串存储成vec
        // 然后翻转vec之后，每个分割的term使用trim修剪
        // 再组合成String返回

        String::new()

        // 方法2
        // 迭代s，因为单词是不可能由空格组成的，所以只要遇到空格就可以确定一个单词
        // 将这个单词存储到vec中
        // 翻转vec，然后除最后一个单词以外其他单词后面都添加一个空格
        // 最后collect返回
    }
}