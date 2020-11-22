use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    // num的范围在[0, 2^30 - 1]之间，不会发生整数溢出。
    pub fn exchange_bits(num: i32) -> i32 {
        // 方法1
        // 将num转换为2进制数组，然后两两交换，再转换成数字
        // Passed 0ms 2.1mb
        // let mut num = num;
        // let mut bits = Vec::new();
        // while num > 0 {
        //     bits.push(num & 1);
        //     num >>= 1;
        // }
        // if bits.len() & 1 == 1 { bits.push(0); }
        // bits.chunks_mut(2).for_each(|chunk| chunk.swap(0, 1));
        // while let Some(bit) = bits.pop() {
        //     num <<= 1;
        //     num += bit;
        // }
        // num

        // 方法2
        // 利用1010(A) 1010...提取偶数的，然后右移1
        // 利用0101(5) 0101...提取奇数的，然后左移1
        // 然后把两个数or起来完成交换
        // Passed 0ms 2.1mb
        (num & 0xaaaaaaaa_u32 as i32) >> 1 | (num & 0x_55555555) << 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::exchange_bits(0), 0);
    assert_eq!(Solution::exchange_bits(2), 1);
    assert_eq!(Solution::exchange_bits(1), 2);
    assert_eq!(Solution::exchange_bits(3), 3);
    assert_eq!(Solution::exchange_bits(0b_0111 as i32), 0b_1011 as i32);
    assert_eq!(Solution::exchange_bits(0b_0111_1111_1111_1111_1111_1111_1111_1111 as i32), -1073741825);
}