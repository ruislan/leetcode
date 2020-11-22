use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        // 方法1
        // 规则是1-3可以选，那么只要把4这个数字留给对手就行了，这样他无论怎么选，都要输
        // 按照这个方法，不管多大的数字，我们只要保持每轮都把4的倍数留给对手，我们就能稳赢
        // 所以判断开始的数字，我们是不是能稳赢，就看对手是不是直接给我们4的倍数
        // Passed 0ms 2mb
        n % 4 != 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_win_nim(4), false);
    assert_eq!(Solution::can_win_nim(1), true);
    assert_eq!(Solution::can_win_nim(8), false);
    assert_eq!(Solution::can_win_nim(12), false);
    assert_eq!(Solution::can_win_nim(16), false);
    assert_eq!(Solution::can_win_nim(20), false);
    assert_eq!(Solution::can_win_nim(21), true);
}