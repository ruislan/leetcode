use crate::q::Solution;

impl Solution {
pub fn add_digits(num: i32) -> i32 {
        // 方法1
        // if num < 10 { return num; }
        // let mut num = num;
        // let mut new_num = 0;
        // while num >= 10 {
        //     new_num += num % 10;
        //     num /= 10;
        // }
        // new_num += num % 10;
        // return self::add_digits(new_num);

        // 方法2
        (num - 1) % 9 + 1
    }
}