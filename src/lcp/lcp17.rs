use crate::lcp::Solution;

#[allow(unused)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        // 方法1
        // "A" 运算：使 x = 2 * x + y；
        // "B" 运算：使 y = 2 * y + x。
        // 初始x = 1, y = 0
        // 迭代s，根据A，B逐个计算，最后得到x,y，返回x + y
        // 利用s.bytes().fold((1,0), |acc, cmd| match cmd { b'A' => ..., b'B' => ...}).map(|acc| acc.0 + acc.1)可以变得简洁优雅
        // Passed 0ms 1.9mb
        // let (x, y) = s.bytes().fold((1, 0), |(x, y), cmd|
        //     match cmd {
        //         b'A' => ((x << 1) + y, y),
        //         _ => (x, (y << 1) + x),
        //     });
        // x + y

        // 方法2
        // 观察几个结果，
        // ''  x=1,y=0, x+y=1                 , '' x=1,y=0, x+y=1
        // 'A' x=2,y=0, x+y=2                 , 'A' x=2,y=0, x+y=2
        // 'AB' x=2,y=2,x+y=4                 , 'AA'x=4,y=0, x+y=4
        // 'ABA' x=6,y=2,x+y=8                , 'AAA'x=8,y=0,x+y=8
        // 'ABAB' x=6,y=10, x+y=16            , 'AAAA'x=16,y=0,x+y=16
        // 'ABABA'x=22,y=10,x+y=32            , 'AAAAA'x=32,y=0,x+y=32
        // 'ABABAB'x=22,y=42,x+y=64           , 'AAAAAA'x=64,y=0,x+y=64
        // 基本可以看出来了，不管怎么变，每多一个字符，结果就翻一倍
        // Passed 0ms 2mb
        // 2_i32.pow(s.len() as u32)

        // 方法3
        // 方法2的装逼版本
        // Passed 0ms 1.8mb
        1 << s.len()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::calculate("".to_string()), 1);
    assert_eq!(Solution::calculate("A".to_string()), 2);
    assert_eq!(Solution::calculate("AB".to_string()), 4);
    assert_eq!(Solution::calculate("ABA".to_string()), 8);
    assert_eq!(Solution::calculate("AAAA".to_string()), 16);
    assert_eq!(Solution::calculate("BABA".to_string()), 16);
    assert_eq!(Solution::calculate("ABAA".to_string()), 16);
    assert_eq!(Solution::calculate("BBBB".to_string()), 16);
}