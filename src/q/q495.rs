use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        // 方法1
        // 迭代时间序列，如果这个时间点在停止时间之前或者相等，
        // 则只需要计算这个时间点和开始时间得差（不用+1，因为后续会用这个停止时间作为开始时间，+1就重复计算了）
        // 否则，就只需要累加一个duration即可，最后再累加一次duration，即为总时间
        // AC 4ms 2.2mb 38/38
        let mut ans = 0;
        let mut start = time_series[0];
        let n = time_series.len();
        for i in 1..n {
            if time_series[i] <= start + duration - 1 {
                ans += time_series[i] - start;
            } else {
                ans += duration;
            }
            start = time_series[i];
        }
        ans + duration
    }
}