use crate::q::Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        // 方法1：
        // 转换成字符串，
        // 按从左到右的顺序找到第一个6，改变成9，无6不改变即可
        // Passed 0ms 2.1mb
        match num.to_string().replacen("6", "9", 1).parse::<i32>() {
            Ok(n) => n,
            Err(_) => num,
        }

        // 方法2：
        // 直接数字，按从左到右的顺序找到第一个6，改变成9，无6不改变即可
        // Passed 0ms 2.1mb
        // for i in (0..5).rev() {
        //     if (num / 10_i32.pow(i)) % 10 == 6 {
        //         return num + 3 * 10_i32.pow(i);
        //     }
        // }
        // num
    }
}

#[test]
fn test_q1323() {
    assert_eq!(Solution::maximum69_number(9669), 9969);
    assert_eq!(Solution::maximum69_number(9969), 9999);
    assert_eq!(Solution::maximum69_number(9999), 9999);
    assert_eq!(Solution::maximum69_number(9), 9);
    assert_eq!(Solution::maximum69_number(6), 9);
    assert_eq!(Solution::maximum69_number(66666), 96666);
}