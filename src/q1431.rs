use crate::Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // 方法1
        // 找出candies中最大的那个数max，
        // 然后迭代candies，如果candies[i] + extra_candies >= max，则map为true，再collect返回
        let max = *candies.iter().max().unwrap_or(&0);
        candies.into_iter().map(|x| x + extra_candies >= max).collect()
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::kids_with_candies(vec![], 0), vec![]);
    assert_eq!(Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3), vec![true, true, true, false, true]);
    assert_eq!(Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1), vec![true, false, false, false, false]);
    assert_eq!(Solution::kids_with_candies(vec![12, 1, 12], 10), vec![true, false, true]);
}