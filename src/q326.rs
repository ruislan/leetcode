use crate::Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // 方法1
        // 循环n从0开始，n * n * n
        // 如果结果大于n，break，返回false
        // 如果结果等于n,返回true
        // Passed 16ms 2mb
        // if n <= 0 { return false; }
        // let n = n as u32;
        // for i in 0..=n {
        //     let num = 3_u32.pow(i);
        //     if num == n { return true; }
        //     if num > n { break; }
        // }
        // false

        // 方法1变种
        // Passed 8ms 2mb
        // if n <= 0 { return false; }
        // let mut n = n;
        // while n % 3 == 0 { n /= 3; }
        // n == 1

        // 方法2
        // log 3为底，放入n，结果是整数返回true，否则false
        // Passed 0-8ms 2.5mb
        if n <= 0 { return false; }
        let n = (n as f64).log(3_f64);
        (n - n.round()).abs() < 1e-12_f64

        // 方法3
        // 直接用3的19次方来取n的余数，要等于0
        // 4-8ms 1.9mb
        // n > 0 && 1162261467 % n == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_three(-1), false);
    assert_eq!(Solution::is_power_of_three(0), false);
    assert_eq!(Solution::is_power_of_three(1), true);
    assert_eq!(Solution::is_power_of_three(3), true);
    assert_eq!(Solution::is_power_of_three(9), true);
    assert_eq!(Solution::is_power_of_three(26), false);
    assert_eq!(Solution::is_power_of_three(27), true);
    assert_eq!(Solution::is_power_of_three(1594322), false);
    assert_eq!(Solution::is_power_of_three(1594323), true);
    assert_eq!(Solution::is_power_of_three(1162261466), false);
    assert_eq!(Solution::is_power_of_three(1162261467), true);
}