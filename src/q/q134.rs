use crate::q::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // 方法1
        // 模拟从每一个gas[i]出发的情况
        // Passed 12ms 2mb O(n^2)
        // fn check_range(mut power: &mut i32, gas: &Vec<i32>, cost: &Vec<i32>, range: std::ops::Range<usize>) -> bool {
        //     for i in range {
        //         *power = *power + gas[i] - cost[i];
        //         if *power < 0 { return false; }
        //     }
        //     true
        // }
        // for start in 0..gas.len() {
        //     let mut power = 0;
        //     if check_range(&mut power, &gas, &cost, start..gas.len()) && check_range(&mut power, &gas, &cost, 0..start) {
        //         return start as i32;
        //     }
        // }
        // -1

        // 方法2
        // 方法1其实重复计算了很多次从gas[i]到gas[j]之间的加油站的加减，我们其实可以忽略这些计算
        // 因为如果x位于gas[i]加油站，如果无法到达gas[j]加油站，那么从i..j之间的任何一个加油站出发，都不能到达gas[j]
        // 所以我们如果找到j这个位置不能达到，那么我们就从j + 1继续，这样就能过滤掉很多不必要的检查
        // Passed 0ms 2.2mb
        // let mut i = 0;
        // while i < gas.len() {
        //     let mut power = 0;
        //     let mut len = 0;
        //     while len < gas.len() {
        //         let j = (i + len) % gas.len();
        //         power = power + gas[j] - cost[j];
        //         if power < 0 { break; }
        //         len += 1;
        //     }
        //     if len == gas.len() { return i as i32; }
        //     i += len + 1;
        // }
        // -1

        // 方法3
        // 某位大牛的解法，我理解了思路写在这里
        // 我们累加gas[i]和cost[i]的花费到最低值的时候，那个最低值就作为我们的出发点
        // 当然，如果总花费超过了总油量，那肯定就无法到达，如果可以，那么只需要把最低值的那个索引作为出发点即可
        // 简单来说就是在最低谷的那个点作为地平线，那么所有的路都比地平线要高，不会再低了
        // 也就是这类的问题永远要去优先考虑拐点有什么含义
        // Passed 0ms 2.1mb O(n)
        let mut power = 0;
        let mut min_power = i32::MAX;
        let mut start = 0;
        for i in 0..gas.len() {
            power = power + gas[i] - cost[i];
            if power < min_power {
                min_power = power;
                start = i;
            }
        }
        if power >= 0 { ((start + 1) % gas.len()) as i32 } else { -1 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    assert_eq!(Solution::can_complete_circuit(vec![5, 8, 2, 8], vec![6, 5, 6, 6]), 3);
}