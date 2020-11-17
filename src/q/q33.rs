use crate::q::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 虽然题目没有时间要求，最直接能够想到的就是线性探查
        // 但是数据量比较大的情况下，是很慢的，肯定无法通过的
        // 所以这个方法没啥可以说的，略过

        // 方法2
        // 数组被旋转之后分成了两个升序的数组
        // 首先我们需要判断target是在哪个哪个升序的数组中
        // 然后就可以利用标准的二分查找解决问题
        // 我们可以利用target与nums[0]的数据对比来确定他应该是在
        // 左边还是右边那个升序的数组中
        // 假设target在左边，
        //    那么当nums[mid]比nums[0]要小的话：
        //       说明nums[mid]落在右边的升序的数组中，我们需要取左边的区域
        //    当nums[mid]比nums[0]要大或相等的话：
        //       说明nums[mid]落在左边的升序的数组中，我们直接用标准二分来处理
        // 假设target在右边，反之。
        // 这里我们优化一下，可以先判断nums[mid]与nums[0]比，确定它落在哪边
        // 如果它和target在一边，那么就用标准二分处理
        // 如果它和target不在一边，那么就把它赶到target那边去
        // Passed 0ms 2.1mb
        let (mut lo, mut hi) = (0, nums.len() - 1);
        let is_target_lo = target >= nums[0];

        while lo <= hi && hi < nums.len() {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            let is_mid_lo = nums[mid] >= nums[0];
            if is_mid_lo == is_target_lo {
                if nums[mid] > target {
                    hi = mid.overflowing_sub(1).0;
                } else {
                    lo = mid + 1;
                }
            } else {
                if is_mid_lo {
                    lo = mid + 1;
                } else {
                    hi = mid.overflowing_sub(1).0;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
}