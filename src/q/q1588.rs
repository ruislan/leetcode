use crate::q::Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        // 方法1
        // 暴力解决，不是最佳解法。
        // 尝试所有的奇数，直到超过arr.len()，因为是连续子数组，所以没有排列组合问题
        // Passed 0ms 2mb
        let (mut len, mut sum) = (1, 0);
        while len <= arr.len() {
            sum += arr.windows(len).map(|win| win.iter().sum::<i32>()).sum::<i32>();
            len += 2;
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![]), 0);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1]), 1);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2, 3]), 12);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2, 3, 4, 5]), 57);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
}