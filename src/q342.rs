use crate::Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        // 方法1
        // 循环n从0开始，n.pow(4)
        // 如果结果大于num，break，返回false
        // 如果结果等于num,返回true
        // Passed 0ms 2mb
        // let mut num = num;
        // if num <= 0 { return false; }
        // while num % 4 == 0 {
        //     num /= 4;
        // }
        // num == 1

        // 方法2
        // log 4为底，放入num，结果是整数返回true，否则false
        // false
        // 0ms 2.4mb
        if num <= 0 { return false; }
        let num = (num as f64).log(4_f64);
        (num - num.round()).abs() < 1e-12_f64
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_power_of_four(-1), false);
    assert_eq!(Solution::is_power_of_four(0), false);
    assert_eq!(Solution::is_power_of_four(1), true);
    assert_eq!(Solution::is_power_of_four(2), false);
    assert_eq!(Solution::is_power_of_four(3), false);
    assert_eq!(Solution::is_power_of_four(4), true);
    assert_eq!(Solution::is_power_of_four(6), false);
    assert_eq!(Solution::is_power_of_four(16), true);
    assert_eq!(Solution::is_power_of_four(1073741823), false);
    assert_eq!(Solution::is_power_of_four(1073741824), true);
}