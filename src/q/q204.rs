use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // 方法1
        // let mut count = 0;
        // for i in 2..n {
        //     let mut is_prime = true;
        //     let mut j = 2;
        //     while j * j <= i {
        //         if i % j == 0 {
        //             is_prime = false;
        //             break;
        //         }
        //         j += 1;
        //     }
        //     if is_prime {
        //         count += 1;
        //     }
        // }
        // count
        
        // 方法2
        let n = n as usize;
        let mut nums = vec![true; n];
        let mut i = 2;
        while i * i <= n {
            if nums[i] {
                let mut j = i * i;
                while j < n {
                    nums[j] = false;
                    j += i;
                }
            }
            i += 1;
        }

        let mut count = 0;
        for i in 2..n {
            if nums[i] { count += 1; }
        }

        count
    }
}