use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_number(s: String) -> bool {
        // 方法1
        // 首先判断字符串里面是否有非e的字符，有的话就直接返回失败
        // 然后利用字符串解析成数字，成功解析成数字的就是数字
        // AC 0ms 2mb
        for ch in s.chars() {
            if ch.is_alphabetic() && (ch != 'e' || ch != 'E') { return false; }
        }
        match s.parse::<f64>() {
            Ok(x) => true,
            Err(_) => false
        }
    }
}