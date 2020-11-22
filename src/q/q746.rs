use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut f0, mut f1) = (cost[0], cost[1]);
        for i in 2..cost.len() {
            let f2 = cost[i] + std::cmp::min(f0, f1);
            f0 = f1;
            f1 = f2;
        }
        std::cmp::min(f0, f1)
    }
}