use crate::q::Solution;

impl Solution {
    // 方法名max_profit与q121相同，更名为max_profit_ii
    pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut buy_p = i32::max_value();
        let mut last_p = 0;
        for price in prices {
            if price < buy_p && last_p == 0 { buy_p = price; } else if price > last_p { last_p = price; } else {
                total += last_p - buy_p;
                last_p = 0;
                buy_p = price;
            }
        }
        if last_p > buy_p { total += last_p - buy_p; }
        total
    }
}