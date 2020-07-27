use crate::q::Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        // 方法1
        // 设置一个初始的max = 0; count = 0;
        // 查找当前字符与上一个字符是否相等，相等就count += 1，如果不相等，就令count = 1，
        // 然后比较count和max，比max大就替换
        // 利用for i in 1..s.len():
        //       if &s[i - 1] == &s[i]:
        //          count += 1;
        //          max = max.max(count);
        //       else ...
        // Passed 0ms 2mb
        // let (mut max, mut count) = (1, 1);
        // (1..s.len()).for_each(|i| {
        //     if &s[(i - 1)..i] == &s[i..(i + 1)] { count += 1; } else { count = 1; }
        //     max = max.max(count);
        // });
        // max

        // 方法2
        // 试试看能不能利用partition来分组，然后直接取最大长度的组？这个不确定
        // 用windows函数试试
        s.as_bytes().windows(2).fold((1, 1), |(max, mut acc), x| {
            if x[0] == x[1] { acc += 1; } else { acc = 1; }
            (max.max(acc), acc)
        }).0
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::max_power("".to_string()), 0);
    assert_eq!(Solution::max_power("leetcode".to_string()), 2);
    assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
    assert_eq!(Solution::max_power("triplepillooooow".to_string()), 5);
    assert_eq!(Solution::max_power("hooraaaaaaaaaaay".to_string()), 11);
    assert_eq!(Solution::max_power("tourist".to_string()), 1);
    assert_eq!(Solution::max_power("t".to_string()), 1);
    assert_eq!(Solution::max_power("aa".to_string()), 2);
}