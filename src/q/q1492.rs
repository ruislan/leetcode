use crate::q::Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        // 方法1
        // 从1..=n迭代数字，满足n % i == 0的，计数+1，如果计数等于k的，则返回
        // 迭代结束，返回-1
        let mut count = 0;
        for i in 1..=n {
            if n % i == 0 {
                count += 1;
                if count == k { return i; }
            }
        }
        -1
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::kth_factor(0, 0), -1);
    assert_eq!(Solution::kth_factor(12, 3), 3);
    assert_eq!(Solution::kth_factor(7, 2), 7);
    assert_eq!(Solution::kth_factor(4, 4), -1);
    assert_eq!(Solution::kth_factor(1, 1), 1);
    assert_eq!(Solution::kth_factor(1000, 3), 4);
    assert_eq!(Solution::kth_factor(100000000, 100), -1);
}