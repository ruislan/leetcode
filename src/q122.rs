mod q122 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut max_p = 0;
        let mut buy_p = i32::max_value();
        let mut last_p = 0;
        for price in prices {
            if price < buy_p && last_p == 0 { buy_p = price; } else if price > last_p { last_p = price; } else {
                max_p = last_p - buy_p;
                total += max_p;
                last_p = 0;
                buy_p = price;
            }
        }
        if last_p > buy_p { total += last_p - buy_p; }
        total
    }
}