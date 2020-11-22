use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法名max_profit与q121相同，更名为max_profit_ii
    pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
        // 方法1
        // 设置一个最后的买价和上一次的价格
        // 当价格小于买价，且前一次的价格为0时进行购买
        // 否则，当价格大于上一次的价格时，将上一次的价格设置为当前价格
        // 否则，计算利润，清零上一次的价格，且将买价设置为当前价格
        // 最后如果上一次的价格大于买价，则计算总价
        // Passed 0ms 2.1mb
        // let mut total = 0;
        // let mut buy_p = i32::max_value();
        // let mut last_p = 0;
        // for price in prices {
        //     if price < buy_p && last_p == 0 {
        //         buy_p = price;
        //     } else if price > last_p {
        //         last_p = price;
        //     } else {
        //         total += last_p - buy_p;
        //         last_p = 0;
        //         buy_p = price;
        //     }
        // }
        // if last_p > buy_p { total += last_p - buy_p; }
        // total

        // 方法2
        // 方法1是最早的时候做的了，回头看还没摸清楚当时怎么想的，思路感觉有点奇怪
        // 如果按照方法1的思路，应该是这样
        // 设置一个状态记录持仓，设置一个参数记录利润，设置一个参数记录购买价格
        // 迭代prices 从 1 到 len - 1：
        //    如果当前价格大于前一次价格，且处于未持仓状态：
        //       以前一次价格购买
        //    如果当前价格小于前一次价格，且处于已持仓状态：
        //       以前一次价格卖出,计入利润
        // 迭代完成
        // 如果还是持仓状态：
        //    以最后价格卖出
        // Passed 0ms 2.1mb
        // let mut profit = 0;
        // let mut hold = false;
        // let mut buy = 0;
        // for i in 1..prices.len() {
        //     if prices[i] > prices[i - 1] && !hold {
        //         buy = prices[i - 1];
        //         hold = true
        //     }
        //     if prices[i] < prices[i - 1] && hold {
        //         profit += prices[i - 1] - buy;
        //         hold = false
        //     }
        // }
        // if hold {
        //     profit += prices[prices.len() - 1] - buy;
        // }
        // profit

        // 方法3
        // 方法2优化
        // 因为获取利润的段始终在上升段，所以我们可以直接将上升段的差值进行累加即可
        // 迭代prices 从1到最后
        //    如果当前价格大于前面的价格
        //       累加当前价格减去前面的价格
        // 返回利润
        // Passed 0ms 2.3mb
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }
        profit
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::max_profit_ii(vec![1]));
    assert_eq!(1, Solution::max_profit_ii(vec![1, 2]));
    assert_eq!(1, Solution::max_profit_ii(vec![1, 2, 1]));
    assert_eq!(7, Solution::max_profit_ii(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(4, Solution::max_profit_ii(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit_ii(vec![7, 6, 4, 3, 1]));
    assert_eq!(7, Solution::max_profit_ii(vec![6, 1, 3, 2, 4, 7]));
}