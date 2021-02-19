use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 遇到0，翻转向前K个，直到最后
        // 如果最后无法翻转，则返回-1
        // 否则返回翻转次数
        // Kotlin AC 552ms 44mb
        // Rust 超时
        // Kotlin接受而Rust超时说明这个解法本身是不被认可的吧，看来我们要优化一下
        // let mut a = a;
        // let n = a.len();
        // let k = k as usize;
        //
        // let mut answer = 0;
        // let mut lo = 0;
        // while lo < n {
        //     if a[lo] == 0 {
        //         if lo + k > n {
        //             return -1;
        //         } else {
        //             for i in lo..lo+k {
        //                 a[i] = a[i] ^ 1;
        //             }
        //             answer += 1;
        //         }
        //     }
        //     lo += 1;
        // }
        // answer

        // 方法2
        // 注意到数组中只有0和1，那么也就是翻转两次等于不翻转，
        // 我们假设检查到了第i个数字，那么a[i - k]个数字可能翻转了，也可能没有翻转
        // 我们需要知道是否翻转了，这样确定我们是否需要翻转当前数字
        // AC 16ms 2.1mb
        let n = a.len();
        let mut a = a;
        let k = k as usize;
        let mut rev_count = 0;
        let mut answer = 0;
        for i in 0..n {
            if i >= k && a[i - k] > 1 {
                rev_count ^= 1;
                a[i - k] -= 2;
            }
            if rev_count == a[i] {
                if i + k > n { return -1; }
                a[i] += 2;
                rev_count ^= 1;
                answer += 1;
            }
        }
        answer
    }
}