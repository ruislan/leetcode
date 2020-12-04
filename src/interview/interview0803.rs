use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        // 方法1
        // 暴力解决
        // 时间：O(n)，空间：O(1)
        // Passed 0ms 2.2mb
        // for i in 0..nums.len() {
        //     if nums[i] == i as i32 { return nums[i]; }
        // }
        // -1

        // 方法2
        // 利用其有序的特性，那么就可以用二分查找
        // 先二分向左找，如果没找到，向右
        // 时间：O(logn)，空间：O(logn)
        // Passed 0ms 2.1mb
        fn find(nums: &Vec<i32>, lo: i32, hi: i32) -> i32 {
            if lo > hi { return -1; }
            let mid = lo + (hi - lo) / 2;
            let answer = find(nums, lo, mid - 1);
            if answer != -1 {
                answer
            } else if nums[mid as usize] == mid {
                mid
            } else {
                find(nums, mid + 1, hi)
            }
        }
        find(&nums, 0, (nums.len() - 1) as i32)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_magic_index(vec![3, 4, 5, 5, 5, 5]), 5);
}