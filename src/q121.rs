mod q121 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 方法1
        // if prices.len() < 2 { return 0; }
        // let (mut min, mut max, mut res) = (prices[0], 0, 0);
        // for i in 1..prices.len() {
        //     let num = prices[i];
        //     if num > min && (num - min) > res { res = num - min; }
        //     if num < min {
        //         min = num;
        //         max = 0;
        //     }
        // }
        // res

        // 方法2
        let (mut min, mut res) = (i32::max_value(), 0);
        for num in prices {
            if num < min { min = num; } else if num - min > res { res = num - min; }
        }
        res
    }
}