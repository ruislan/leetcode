use crate::q::Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut negative = false;
        let mut res = 0i64;
        for (i, ch) in str.trim().chars().enumerate() {
            if ch == '+' && i == 0 {
                continue;
            }
            if ch == '-' && i == 0 {
                negative = true;
                continue;
            }
            if !ch.is_digit(10) {
                break;
            }
            res = 10i64 * res + ch.to_digit(10).unwrap() as i64;
            if negative {
                if -res < i32::min_value() as i64 {
                    return i32::min_value();
                }
            } else {
                if res > i32::max_value() as i64 {
                    return i32::max_value();
                }
            }
        }
        if negative {
            -res as i32
        } else {
            res as i32
        }
    }
}

#[test]
fn test_atoi() {
    assert_eq!(-2147483648, Solution::my_atoi("-2147483648".to_string()));
    assert_eq!(131204, Solution::my_atoi("+00131204".to_string()));
    assert_eq!(10, Solution::my_atoi("010".to_string()));
    assert_eq!(0, Solution::my_atoi("+-2".to_string()));
    assert_eq!(
        -1,
        Solution::my_atoi("-000000000000000000000000000000000000000000000000001".to_string())
    );
    assert_eq!(12345678, Solution::my_atoi("  0000000000012345678".to_string()));
    assert_eq!(42, Solution::my_atoi("42".to_string()));
    assert_eq!(-42, Solution::my_atoi("   -42".to_string()));
    assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
    assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
    assert_eq!(3, Solution::my_atoi("3*14".to_string()));
    assert_eq!(3, Solution::my_atoi("3.14159".to_string()));
    assert_eq!(0, Solution::my_atoi(".1".to_string()));
    assert_eq!(0, Solution::my_atoi("++".to_string()));
    assert_eq!(1, Solution::my_atoi("1+".to_string()));
    assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
    assert_eq!(
        i32::min_value(),
        Solution::my_atoi("-912834723320000000000000000000".to_string())
    );
    assert_eq!(
        i32::max_value(),
        Solution::my_atoi("912834723320000000000000000000".to_string())
    );
}