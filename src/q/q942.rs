use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        // 方法1
        // 最小值从0开始，最大值从s.len()开始
        // 结果比原长度多1，所以最大值是从s.len()开始
        // 迭代s，遇到是D的，放入最大值，然后最大值-1
        //       遇到是I的，放入最小值，然后最小值+1
        // 最后，因为多一，所以需要多放入一个值，放哪个都行，因为最大值和最小值已经相等
        // Passed 4ms 2.1mb
        let (mut min, mut max) = (0, s.len() as i32);
        let mut answer = Vec::new();
        for x in s.into_bytes() {
            if x == 73 {
                answer.push(min);
                min += 1;
            } else {
                answer.push(max);
                max -= 1;
            }
        }
        answer.push(min);
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::di_string_match("I".to_string()), vec![0, 1]);
    assert_eq!(Solution::di_string_match("D".to_string()), vec![1, 0]);
    assert_eq!(Solution::di_string_match("III".to_string()), vec![0, 1, 2, 3]);
    assert_eq!(Solution::di_string_match("DDD".to_string()), vec![3, 2, 1, 0]);
    assert_eq!(Solution::di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
    assert_eq!(Solution::di_string_match("DIDI".to_string()), vec![4, 0, 3, 1, 2]);
}