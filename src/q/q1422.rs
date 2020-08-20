use crate::q::Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        // 方法1
        // 暴力解决方法，分割点从1..s.len() - 1，得到所有的值，取最大值
        // Passed 0ms 2mb
        // let mut max = 0;
        // for i in 1..s.len() {
        //     let zeros = &s[..i].bytes().filter(|&ch| ch == b'0').count();
        //     let ones = &s[i..].bytes().filter(|&ch| ch == b'1').count();
        //     max = max.max(zeros + ones);
        // }
        // max as i32

        // 方法2
        // 向右求出0的前缀和zeros，向左求出1的前缀和ones
        // zeros和ones的相同位置相加得到数组arr，skip头尾，返回arr[i]中的第二大值即可
        // 这个idea不行，会被 “10”这个案例给搞死

        // 方法3
        // 先统计1的个数
        // 然后从左到右 - 1，遇到0，个数加1，遇到1，个数减1，取最大值
        let mut sum = s.bytes().filter(|&ch| ch == b'1').count();
        let mut max = 0;
        s[..(s.len() - 1)].bytes().for_each(|ch| {
            if ch == b'0' { sum += 1; } else { sum -= 1; }
            max = max.max(sum);
        });
        max as i32
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::max_score("".to_string()), 0);
    assert_eq!(Solution::max_score("011101".to_string()), 5);
    assert_eq!(Solution::max_score("00111".to_string()), 5);
    assert_eq!(Solution::max_score("1111".to_string()), 3);
    assert_eq!(Solution::max_score("0000".to_string()), 3);
    assert_eq!(Solution::max_score("00".to_string()), 1);
    assert_eq!(Solution::max_score("11".to_string()), 1);
    assert_eq!(Solution::max_score("01".to_string()), 2);
    assert_eq!(Solution::max_score("10".to_string()), 0);
}