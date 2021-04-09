use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // 方法1
        // 二分搜索
        // 中间数据与首数字first = nums[0]比较
        // 如果比first相等或则大，说明最小数据在右边lo = mid + 1;
        // 如果比first小，则说明最小数据在左边hi = mid - 1;
        // 这里有个特殊情况，就是正常升序会导致lo在最右边，
        // 所以这个情况下，我们的最小值就是第一个nums[0]
        // AC 0ms 2mb
        // let n = nums.len();
        // let mut lo = 0;
        // let mut hi = n - 1;
        // while lo <= hi && hi < n {
        //     let mid = lo + (hi - lo) / 2;
        //     if nums[mid] >= nums[0] {
        //         lo = mid + 1;
        //     } else {
        //         hi = mid - 1;
        //     }
        // }
        // if lo >= n { return nums[0]; }
        // nums[lo]

        // 方法2
        // 中间数字与当前范围的hi进行比较
        // 如果nums[mid] > nums[hi]：
        //    说明最小数据在右边
        // 否则
        //    说明最小数据在左边
        // AC 0ms 2mb
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        while lo < hi && hi < n {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
}