use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // 方法1
        // 1. 求出最喜欢的type的第一个要吃到多少颗。例如：7+4+5+3+1。
        // 这里如果每次都要求和，所以我们现求出前缀和len(n+1)。这样第fd天最少就是min = prefix_sun[type+1]+1
        // 然后每天的最大的个数 * （最喜欢天数 + 1） >= min，这里+1是因为从第0天开始的。（这个算是贪心吗？）
        // 2. 是最喜欢的天数有可能比对应的类型糖果的所有的和都还要大，所以要提前判断。
        // 3. 这里有个陷阱就是注意溢出的问题，所以要把i32提升到i64
        // AC 60ms 10.1mb
        let n = candies_count.len();
        let mut candies_prefix_sum = vec![0_i64; n + 1];
        for i in 0..n {
            candies_prefix_sum[i + 1] = candies_prefix_sum[i] + candies_count[i] as i64;
        }
        let mut answer = vec![false; queries.len()];
        for (i, query) in queries.into_iter().enumerate() {
            let f_type = query[0] as usize;
            let f_day = query[1] as i64;
            let d_cap = query[2] as i64;
            if f_day >= candies_prefix_sum[f_type + 1] { continue; }
            let min = candies_prefix_sum[f_type] + 1;
            if d_cap * (f_day + 1) >= min { answer[i] = true; }
        }
        answer
    }
}