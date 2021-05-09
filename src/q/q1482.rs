use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        // 方法1
        // 首先我们要理解"相邻"的花，也就是必须是k个连在一起的
        // 然后，我们要满足m个"相邻"的花
        // 最简单的方式就是遍历数组，如果相邻，那么就可以形成一束花，只要有m束花
        // 那么题目要求的是最少天数，我们假设从第一天开始看花开的位置
        // 一直到第n天的花，我们的复杂度就要到O(n^2)，必然TLE
        // 而为了降低这个复杂度，我们可以用二分搜索来判断逼近最小的那天，
        // 二分边界就是bloom_day的最小值和最大值，
        // 当mid的结果满足条件的时候，我们将hi = mid，这样我们可以再看有没有更少天数
        // 当mid的结果不满足条件的时候，我们将lo = mid + 1，这说明我们的连续的花不够
        // 这样复杂度可以降低到O(nlogn)
        // AC 28ms 3.7mb
        fn check(day: i32, mut m: i32, k: i32, bloom_day: &Vec<i32>) -> bool {
            let mut bundle = 0;
            for i in 0..bloom_day.len() {
                if bloom_day[i] <= day {
                    bundle += 1;
                } else {
                    bundle = 0;
                }
                if bundle == k {
                    bundle = 0;
                    m -= 1;
                }
                if m == 0 { return true; }
            }
            false
        }

        if m * k > bloom_day.len() as i32 { return -1; }

        let mut lo = *bloom_day.iter().min().unwrap();
        let mut hi = *bloom_day.iter().max().unwrap();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if check(mid, m, k, &bloom_day) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}