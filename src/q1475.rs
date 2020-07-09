use crate::Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 暴力解决
        // res = Vec::new()
        // for i in prices 0..len:
        //     for j in (i + 1)..len:
        //         if prices[i] >= prices[j]:
        //             discount = prices[j], break;
        //         else:
        //             discount = 0;
        //     res[i] = prices[i] - discount;
        // return res
        let mut res = vec![0; prices.len()];
        (0..prices.len()).for_each(|i| {
            res[i] = prices[i] - *prices.iter().skip(i).find(|&&x| x <= prices[i]).unwrap_or(&0);
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