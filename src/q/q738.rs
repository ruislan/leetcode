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
        // if n < 10 { return n; }

        // 方法2
        // 逐个数字比较，只要是递增(x <= y)都没问题y是低位，x是高位
        // 当出现第一个递减(x > y)的情况，那么意味着后面的都要变成9,自己再减去1
        // 这里可以排除0 - 1的情况，因为0是最小的，意味着如果当前位置的数字是0
        // 那么就只能是 x <= y的情况，而后面都变成9的意思就是我最高位少1，那么低位最大的就都应该是9
        // Passed 0ms 2.1mb
        let mut n = n;
        let mut nums = Vec::new();
        while n > 0 {
            nums.push(n % 10);
            n /= 10;
        }
        let mut answer = 0;
        let mut offset = 1;
        for i in 0..nums.len() - 1 {
            let y = nums[i];
            let x = nums[i + 1];
            if x <= y {
                answer += y * offset;
            } else {
                nums[i + 1] = x - 1;
                answer = offset * 10 - 1;
            }
            offset *= 10;
        }
        answer + nums[nums.len() - 1] * offset
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
