use crate::q::Solution;

impl Solution {
    // 方法与q104重复，更名为max_brackets_depth
    pub fn max_brackets_depth(s: String) -> i32 {
        // 方法1
        // 设置最大值为max = 0,
        // '('括号的时候设置为+1，将数量与最大值比较，取最大值
        // ')'括号的时候数量-1
        // Passed 0ms 2mb
        let mut max = 0;
        let mut count = 0;
        s.chars().for_each(|ch| {
            match ch {
                '(' => {
                    count += 1;
                    max = max.max(count);
                }
                ')' => count -= 1,
                _ => ()
            }
        });
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_brackets_depth("1".to_string()), 0);
    assert_eq!(Solution::max_brackets_depth("1+(2*3)/(2-1)".to_string()), 1);
    assert_eq!(Solution::max_brackets_depth("(1)+((2))+(((3)))".to_string()), 3);
    assert_eq!(Solution::max_brackets_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    assert_eq!(Solution::max_brackets_depth("8*((1*(5+6))*(8/6))".to_string()), 3);
}