use crate::q::Solution;

impl Solution {
    // 方法名search与q33重复了，更改为binary_search
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 标准的二分查找解决就行了
        // 唯一需要注意的就是Rust的索引是usize
        // 所以我们的hi会造成overflow的可能，需要处理一下
        // Passed 8ms 2.1mb
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo <= hi && hi < nums.len() {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                hi = mid.overflowing_sub(1).0;
            } else {
                lo = mid + 1;
            }
        }
        -1
    }
}