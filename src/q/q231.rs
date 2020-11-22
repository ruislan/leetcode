use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // 方法1
        // let mut left = 0;
        // let mut right = 30;
        // if n > (1 << right) || n <= 0 {
        //     return false;
        // }
        // while left <= right {
        //     let mid = (right + left) / 2;
        //     let x = 1 << mid;
        //     if x == n {
        //         return true;
        //     }
        //     if x > n {
        //         right = mid - 1;
        //     } else {
        //         left = mid + 1;
        //     }
        // }
        // false

        // 方法2
        // n > 0 && n.count_ones() == 1

        // 方法3
        n > 0 && (n & (n - 1)) == 0
    }
}

#[test]
fn test_q231() {
    assert_eq!(false, Solution::is_power_of_two(1073741825));
}