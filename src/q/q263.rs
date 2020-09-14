use crate::q::Solution;

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        // 方法1
        // if num == 0 { return false; }
        // if num == 1 { return true; }
        // if num % 2 != 0 && num % 3 != 0 && num % 5 != 0 {
        //     return false;
        // }
        // if num % 2 == 0 {
        //     return Self::is_ugly(num / 2);
        // }
        // if num % 3 == 0 {
        //     return Self::is_ugly(num / 3);
        // }
        // if num % 5 == 0 {
        //     return Self::is_ugly(num / 5);
        // }
        // false
        let mut num = num;
        if num == 0 { return false; }
        while num % 2 == 0 {
            num /= 2;
        }
        while num % 3 == 0 {
            num /= 3;
        }
        while num % 5 == 0 {
            num /= 5;
        }
        num == 1
    }
}