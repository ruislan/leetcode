use crate::q::Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        // 方法1：
        // 判断一个数是否有0的方法是循环取10的余数，为0则有，不为0则没有
        // 从1开始到当前数n，a为迭代当前数i，b为n-i，判断两个数是否都为无0的
        // Passed 0ms 2.1mb
        // 'outer: for i in 1..=(n / 2) {
        //     let mut a = i;
        //     while a > 0 {
        //         if a % 10 == 0 { continue 'outer; }
        //         a /= 10;
        //     }
        //     let mut b = n - i;
        //     while b > 0 {
        //         if b % 10 == 0 { continue 'outer; }
        //         b /= 10;
        //     }
        //     return vec![i, n - i];
        // }
        // vec![]

        // 方法2：转换成string，检查是否含有0字符
        // Passed 0ms 2.1mb
        (1..=(n / 2)).find(|&i| !(i.to_string() + &(n - i).to_string()).contains('0')).map_or(vec![], |i| vec![i, n - i])
    }
}

#[test]
fn test_q1317() {
    assert_eq!(Solution::get_no_zero_integers(10000), vec![1, 9999]);
    assert_eq!(Solution::get_no_zero_integers(101010), vec![1111, 99899]);
}