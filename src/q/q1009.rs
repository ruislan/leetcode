use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        // 方法1
        // if n == 0 { return 1; }
        // let mut n = n;
        // let mut bits = Vec::new();
        // while n != 0 {
        //     if n % 2 == 1 { bits.push(0); } else { bits.push(1); }
        //     n /= 2;
        // }
        // let mut res = 0;
        // for i in 0..bits.len() {
        //     res += 2_i32.pow(i as u32) * bits[i];
        // }
        // res

        // 方法2
        if n == 0 { return 1; }
        let mut num = 1;
        while num <= n {
            num <<= 1;
        }
        num - 1 - n
    }
}