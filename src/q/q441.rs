use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // 方法1
        // let mut sum = n;
        // for i in 1..i32::max_value() {
        //     if sum > i {
        //         sum -= i;
        //     } else if sum == i {
        //         return i;
        //     } else {
        //         return i - 1;
        //     }
        // }
        // 0

        // 方法2
        ((2f64 * n as f64 + 0.25f64).sqrt() - 0.5f64) as i32
    }
}