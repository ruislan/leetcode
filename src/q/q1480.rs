use crate::q::Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 当前整个数字就是前缀和
        let mut nums = nums;
        (1..nums.len()).for_each(|i| nums[i] += nums[i - 1]);
        nums
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::running_sum(vec![]), vec![]);
    assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(Solution::running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    assert_eq!(Solution::running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    assert_eq!(Solution::running_sum(vec![-100000, -9999, 9998, 1, 184921]), vec![-100000, -109999, -100001, -100000, 84921]);
}