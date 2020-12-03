use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // 方法1
        // 埃筛
        // Passed 20ms 3.3mb
        // let n = n as usize;
        // let mut primes = vec![true; n];
        // let mut answer = 0;
        // for x in 2..n {
        //     if primes[x] {
        //         answer += 1;
        //         for y in ((x + x)..n).step_by(x) {
        //             primes[y] = false;
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 改进方法1
        // Passed 8ms 3.3mb
        let n = n as usize;
        let mut primes = vec![true; n];
        let mut i = 2;
        while i * i <= n {
            if primes[i] {
                let mut j = i * i;
                while j < n {
                    primes[j] = false;
                    j += i;
                }
            }
            i += 1;
        }

        let mut answer = 0;
        for i in 2..n {
            if primes[i] { answer += 1; }
        }
        answer
    }
}