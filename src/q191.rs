use crate::Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        // 方法1
        // 利用库函数n.count_ones()
        // Passed 0ms 2mb
        // n.count_ones() as i32

        // 方法2
        // 辗转相除2，余数为1的，记录余数为1的返回
        let mut count = 0;
        let mut n = n;
        while n > 0 {
            count += n & 1;
            n >>= 1;
        }
        count as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::hamming_weight(u32::from_str_radix("00000000000000000000000000001011", 2).unwrap_or(0)), 3);
    assert_eq!(Solution::hamming_weight(u32::from_str_radix("11111111111111111111111111111111", 2).unwrap_or(0)), 32);
    assert_eq!(Solution::hamming_weight(u32::from_str_radix("00000111110000000000000000001011", 2).unwrap_or(0)), 8);
}