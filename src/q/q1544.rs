use crate::q::Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        // 方法1
        // 构建一个栈stack,逐步迭代s
        // 如果当前字符与stack.last() 互为相同字符的大小写，则pop()
        // 否则压入stack
        // 最后将stack组合成字符串即是结果
        // Passed 0ms 2mb
        let mut stack = Vec::new();
        for ch in s.bytes() {
            let last = stack.last();
            if last.is_some() && (ch + 32 == *last.unwrap() || ch - 32 == *last.unwrap()) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
        stack.into_iter().map(|ch| ch as char).collect()

        // 方法2
        // 方法1的另外一种写法，看看会不会更好
        // Passed 0ms 2mb
        // 结果是方法1更好
        // let mut stack = Vec::new();
        // for ch in s.chars() {
        //     stack.push(ch);
        //     let n = stack.len();
        //     if n >= 2 && (stack[n - 1] as u8 + 32 == stack[n - 2] as u8 || stack[n - 1] as u8 - 32 == stack[n - 2] as u8) {
        //         stack.truncate(n - 2);
        //     }
        // }
        // stack.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::make_good("".to_string()), "".to_string());
    assert_eq!(Solution::make_good("C".to_string()), "C".to_string());
    assert_eq!(Solution::make_good("CC".to_string()), "CC".to_string());
    assert_eq!(Solution::make_good("cC".to_string()), "".to_string());
    assert_eq!(Solution::make_good("Cc".to_string()), "".to_string());
    assert_eq!(Solution::make_good("CcC".to_string()), "C".to_string());
    assert_eq!(Solution::make_good("leEeetcode".to_string()), "leetcode".to_string());
    assert_eq!(Solution::make_good("abBAcC".to_string()), "".to_string());
}