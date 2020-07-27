use crate::q::Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        // 方法1
        // n如果是奇数直接返回a，如果n是偶数返回奇数个a和1个b
        // Passed 0ms 2mb
        // if n & 1 == 0 { "a".repeat(n as usize - 1) + "b" } else { "a".repeat(n as usize) }
        // Passed 0ms 1.9mb
        "a".repeat(n as usize - 1) + if n & 1 != 0 { "a" } else { "b" }
    }
}


#[test]
fn test_q1374() {
    assert_eq!(Solution::generate_the_string(4), "aaab".to_string());
    assert_eq!(Solution::generate_the_string(2), "ab".to_string());
    assert_eq!(Solution::generate_the_string(1), "a".to_string());
    assert_eq!(Solution::generate_the_string(7), "aaaaaaa".to_string());
}