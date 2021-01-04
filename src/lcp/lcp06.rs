use crate::lcp::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        // 方法1
        // 迭代数组coins，然后coins[i] / 2 加上 coins[i] % 2，最后sum这些值得到结果
        // Passed 0ms 1.9mb
        coins.iter().map(|coin| coin / 2 + coin % 2).sum()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_count(vec![]), 0);
    assert_eq!(Solution::min_count(vec![4, 2, 1]), 4);
    assert_eq!(Solution::min_count(vec![2, 3, 10]), 8);
}