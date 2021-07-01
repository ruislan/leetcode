use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        // 方法1
        // 贪心算法，排序costs，从最小的开始买
        // AC 28ms 3.7mb
        let mut costs = costs;
        costs.sort_unstable();
        let mut answer = 0;
        let mut used = 0;
        for i in 0..costs.len() {
            if used + costs[i] <= coins {
                answer += 1;
                used += costs[i];
            } else {
                break;
            }
        }
        answer
    }
}
