use crate::q::Solution;

impl Solution {
    /// 找出素数
    /// 计算素数个数和非素数个数
    /// 组合（素数个数）*组合（非素数个数） 取mod 10^7 + 7
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut unprimes = vec![0; 101];
        for i in 2..=50 {
            for j in 2..=50 {
                if i * j > 100 {
                    break;
                } else {
                    unprimes[(i * j)] += 1;
                }
            }
        }

        let mut primes = 0;
        for x in 2..=n as usize {
            if unprimes[x] == 0 { primes += 1; }
        }

        let mut sum = 1_i64;
        let m = 10_i64.pow(9) + 7;
        for i in 1..=(n - primes) {
            sum = (sum * i as i64) % m;
        }
        for i in 1..=primes {
            sum = (sum * i as i64) % m;
        }

        sum as i32
    }
}
