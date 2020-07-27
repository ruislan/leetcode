use crate::q::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // 方法1
        // 排序nums，然后取最后两个位置的数字进行计算(nums[i]-1)*(nums[j]-1)
        // let mut nums = nums;
        // nums.sort();
        // (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)

        // 方法2
        // 查找nums中的最大值，和第二大的值
        // 然后代入计算(first-1)*(second-1)
        let (mut first, mut second) = (0, 0);
        nums.iter().for_each(|&x| {
            if x > first {
                second = first;
                first = x;
            } else if x > second { second = x; }
        });
        (first - 1) * (second - 1)
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::max_product(vec![]), 0);
    assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(Solution::max_product(vec![3, 7]), 12);
    // assert_eq!(Solution::max_product(vec![]), 0);
}