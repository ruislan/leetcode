use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        // 方法1 O(n)
        // 迭代low..=high，过滤 x & 1 == 1，count结果返回
        // Passed 392ms 2.1mb
        // (low..=high).filter(|&x| x & 1 == 1).count() as i32

        // 方法2 O(1)
        // 利用公式处理(high - low + high & 1 + low & 1) >> 1
        // (3,7) = (7 - 3 + 1 + 1) / 2 = 3, (3,5,7)
        // (1,7) = (7 - 1 + 1 + 1) / 2 = 4, (1,3,5,7)
        // (8,10) = (10 - 8 + 0 + 0) / 2 = 1, (9)
        // (4,10) = (10 - 4 + 0 + 0) / 2 = 3, (5,7,9)
        // (4,7) = (7 - 4 + 1 + 0) / 2 = 2, (5,7)
        // (2,7) = (7 - 2 + 1 + 0) / 2 = 3, (3,5,7)
        // (3,8) = (8 - 3 + 0 + 1) / 2 = 3, (3,5,7)
        // println!("{} - {} + {} + {} = {}", high, low, high & 1, low & 1, (high - low + (high & 1) + (low & 1)));
        (high - low + (high & 1) + (low & 1)) >> 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_odds(0, 0), 0);
    assert_eq!(Solution::count_odds(3, 7), 3);
    assert_eq!(Solution::count_odds(1, 7), 4);
    assert_eq!(Solution::count_odds(8, 10), 1);
    assert_eq!(Solution::count_odds(4, 10), 3);
    assert_eq!(Solution::count_odds(4, 7), 2);
    assert_eq!(Solution::count_odds(2, 7), 3);
    assert_eq!(Solution::count_odds(3, 8), 3);
}