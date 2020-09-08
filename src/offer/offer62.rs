use crate::offer::Solution;
use std::borrow::BorrowMut;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        // 方法1
        // 暴力解决，利用数组，依次将n放入，然后每隔m删除，直到只剩下一个元素
        // Passed 908ms 2.6mb
        // let mut arr: Vec<i32> = (0..n).collect();
        // let mut i = 0;
        // while arr.len() > 1 {
        //     i = i + m as usize - 1;
        //     i %= arr.len();
        //     arr.remove(i);
        // }
        // arr[0]

        // 方法2
        // 利用约瑟夫环的数学公式pos + m % i, i = 2..n + 1
        // 4ms 2.1mb
        (2..n + 1).fold(0, |acc, i| (acc + m) % i)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::last_remaining(1, 1), 0);
    assert_eq!(Solution::last_remaining(5, 3), 3);
    assert_eq!(Solution::last_remaining(10, 17), 2);
    assert_eq!(Solution::last_remaining(10_i32.pow(5), 10_i32.pow(6)), 4337);
}