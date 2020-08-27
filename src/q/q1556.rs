use crate::q::Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        // 方法1
        // 创建一个计数器count，初始值为0，
        // 创建一个字符数组来记录数字转换成字符和小数点的数组chars
        // 循环，当n > 0 时，
        // 如果counter == 3，chars.push('.'), count = 0,
        // 当前数字num = n % 10， n /= 10， count += 1，chars.push(num)
        // 最后chars倒叙并形成字符串
        String::new()
    }
}