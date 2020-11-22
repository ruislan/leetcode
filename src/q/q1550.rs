use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        // 方法1
        // 创建奇数计数器来计算连续的奇数数量odds
        // 判断奇数的方法是 n & 1 == 1
        // 迭代arr，如果当前是奇数，就令odds += 1
        // 如果奇数计数到3，则直接返回true
        // 迭代结束，返回false
        let mut odds = 0;
        arr.iter().find(|&&x| {
            odds = (odds + 1) * (x & 1);
            odds == 3
        }).is_some()

        // 方法2
        // 库函数window
        // Passed 0ms 2.1mb
        // arr.windows(3).find(|x| x.iter().all(|&n| n & 1 == 1)).is_some()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::three_consecutive_odds(vec![]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![1]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![1, 3]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![1, 2, 3]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![2, 4, 5]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![1, 3, 5]), true);
    assert_eq!(Solution::three_consecutive_odds(vec![1, 2, 1, 1]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![1, 3, 4, 5]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![2, 3, 5, 9]), true);
    assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
    assert_eq!(Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]), true);
}