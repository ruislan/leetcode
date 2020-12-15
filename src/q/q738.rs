use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        // 方法1
        // 暴力法
        // 超时
        // fn is_valid(n: i32) -> bool {
        //     let mut n = n;
        //     let mut y = 1_000_000_000 + 1;
        //     while n > 0 {
        //         let x = n % 10;
        //         n /= 10;
        //
        //         if x > y { return false; }
        //         y = x;
        //     }
        //     true
        // }
        //
        // let mut n = n;
        // while n > 0 {
        //     if is_valid(n) { return n; }
        //     n -= 1;
        // }
        // n

        // 方法2
        // 逐个数字比较，只要是递增(x <= y)都没问题y是低位，x是高位
        // 当出现第一个递减(x > y)的情况，那么意味着后面的都要变成9,自己再减去1
        // 这里可以排除0 - 1的情况，因为0是最小的，意味着如果当前位置的数字是0
        // 那么就只能是 x <= y的情况，而后面都变成9的意思就是我最高位少1，那么低位最大的就都应该是9
        // Passed 0ms 2.1mb
        // let mut n = n;
        // let mut nums = Vec::new();
        // while n > 0 {
        //     nums.push(n % 10);
        //     n /= 10;
        // }
        // let mut answer = 0;
        // let mut offset = 1;
        // for i in 0..nums.len() - 1 {
        //     let y = nums[i];
        //     let x = nums[i + 1];
        //     if x <= y {
        //         answer += y * offset;
        //     } else {
        //         nums[i + 1] = x - 1;
        //         answer = offset * 10 - 1;
        //     }
        //     offset *= 10;
        // }
        // answer + nums[nums.len() - 1] * offset

        // 方法3
        // 这个是某位大佬的思路，我记下来回味一下
        // 由于结果要求各位数字单调递增，那么这些数字必然形如 a0a1a2……an (1 <= a0 <= a1 <= a2 <= …… <= an <= 9)
        // 显然有：
        // a0 a1 a2 …… an      (1 <= a0 <= a1 <= a2 <= …… <= an <= 9)
        // =   a0 *  111……1    + (a1 - a0) *   111……1 + (a2 - a1) * 111……1 + ………… + (an - an-1) * 1
        //            n个1                     n-1个1                n-2个1               1个1
        // 可见最终结果必然是若干个形如 11……11 的数字相加所得。
        // 本题中，最大的n为10^9，所以，可以从111111111开始依次累加，
        // 如果继续累加将导致结果超过n，则去掉一个1继续循环。总累加次数不超过9次。
        // Passed 0ms 2mb
        // 我自己的感悟：
        // 简单来说，就是将后面的数字逐层递加，逐步逼近最大值
        let mut ones = 111111111;
        let mut answer = 0;
        for _ in 0..9 {
            while answer + ones > n {
                ones /= 10;
            }
            answer += ones;
        }
        answer
    }
}

#[test]
pub fn test() {
    assert_eq!(Solution::monotone_increasing_digits(10), 9);
    assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    assert_eq!(Solution::monotone_increasing_digits(332), 299);
    assert_eq!(Solution::monotone_increasing_digits(987_654_321), 899_999_999);
    assert_eq!(Solution::monotone_increasing_digits(107), 99);
    assert_eq!(Solution::monotone_increasing_digits(7_107), 6999);
    assert_eq!(Solution::monotone_increasing_digits(528_357_107), 499_999_999);
}
