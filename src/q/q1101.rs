use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        // 方法1
        // 拿到这道题第一想法可能是贪心，但是实际上是二分查找
        // 简单来说，你可以把它看成在有序序列上猜那个正确的数字
        // 而有序序列就是每日的载重区间，从1个到总重量
        // 而我们需要恰好是d天的最小的重量
        // 例如： [1,2,3,1,1], D = 4
        // total_weight = 8，所以从1..8之间找一个数字，正好4天搞定
        // 这就是二分了，当然，你会发现，数字是4的时候,[1,2;3;1,1]是OK的
        // 但是这并不是最佳，所以即便相等，我们也要尝试找最小的，
        // 所以我们从1..4之间找到2，这个时候[1;2;3..]到3的时候，明显超载了
        // 所以我们要从2..4之间找到3，这个时候[1,2;3;1,1]是OK的
        // 没有更小的了，这就是答案
        // AC 8ms 2.5mb
        let n = weights.len();
        let mut total_weight = weights.iter().sum();
        let mut lo = 0;
        let mut hi = total_weight;
        'outer: while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            
            let mut days = 0;
            let mut each_day_total_weight = 0;
            for i in 0..n {
                if weights[i] > mid {
                    lo = mid + 1;
                    continue 'outer;
                }
                if each_day_total_weight + weights[i] > mid {
                    days += 1;
                    each_day_total_weight = 0;
                }
                each_day_total_weight += weights[i];
            }
            if each_day_total_weight > 0 { days += 1; }

            if days > d {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }
}

#[test]
fn test_q1101() {
    assert_eq!(Solution::ship_within_days(
        vec![1,2,3,1,1], 4 
    ), 3);
}