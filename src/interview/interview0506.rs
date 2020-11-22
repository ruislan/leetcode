use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn convert_integer(a: i32, b: i32) -> i32 {
        // 方法1
        // 计算所有的位数
        // 然后比较不同的位数
        // Passed 0ms 1.9mb
        // fn to_bits(mut n: i32) -> Vec<bool> {
        //     let mut bits = vec![false; 32];
        //     for i in 0..32 {
        //         bits[31 - i] = (n & 1) == 1;
        //         n >>= 1;
        //     }
        //     bits
        // }
        //
        // to_bits(a).iter()
        //     .zip(to_bits(b).iter())
        //     .filter(|(a, b)| a != b)
        //     .count() as i32

        // 方法2
        // 将a和b进行异或，然后统计1的数量
        // 根据异或的特点，a和b是相同的位是可以消除的
        // Passed 0ms 2.1mb
        (a ^ b).count_ones() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::convert_integer(i32::MIN, i32::MAX), 32);
}