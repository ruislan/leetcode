use crate::q::Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 暴力解决
        let mut res = vec![0; prices.len()];
        (0..prices.len()).for_each(|i| {
            res[i] = prices[i] - ((i + 1)..prices.len()).find(|&j| prices[j] <= prices[i]).map_or(0, |j| prices[j]);
        });
        res
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::final_prices(vec![]), vec![]);
    assert_eq!(Solution::final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
    assert_eq!(Solution::final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
}