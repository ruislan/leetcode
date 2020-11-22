use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // 方法1
        // O(n^2)的解法
        // 两层循环，如果结果在lower和upper之间，则计入结果
        // Passed 144ms 2.1mb
        // 这道困难题这么解其实不能算OK的，这里只是暂时让其通过，后来慢慢思考其他解法
        let mut answer = 0;
        let mut lower = lower as i64;
        let mut upper = upper as i64;
        for i in 0..nums.len() {
            let mut sum = 0_i64;
            for j in i..nums.len() {
                sum += nums[j] as i64;
                if sum >= lower && sum <= upper { answer += 1; }
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_range_sum(vec![], 0, 0), 0);
    assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    assert_eq!(Solution::count_range_sum(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864), 3);
}