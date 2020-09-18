use crate::lcp::Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        // 方法1
        // "A" 运算：使 x = 2 * x + y；
        // "B" 运算：使 y = 2 * y + x。
        // 初始x = 1, y = 0
        // 迭代s，根据A，B逐个计算，最后得到x,y，返回x + y
        // 利用s.bytes().fold((1,0), |acc, cmd| match cmd { b'A' => ..., b'B' => ...}).map(|acc| acc.0 + acc.1)可以变得简洁优雅
        0
    }
}