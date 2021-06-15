use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // 方法1
        // 根据描述直接找出i > i - 1 && i > i + 1的位置即可
        // 这里需要注意几个特例，
        // 1.连续增 2.连续降低 3.数据不够3个的
        // O(n)
        // AC 0ms 2mb
        // let n = nums.len();
        // let mut i = 1;
        // let mut answer = 0;
        // while i < n {
        //     if nums[i] > nums[i - 1] {
        //         if i + 1 < n && nums[i] > nums[i + 1] {
        //             return i as i32;
        //         }
        //         answer = i;
        //     }
        //     i += 1;
        // }
        // answer as i32

        // 方法2
        // 二分查找O(logn)
        // 注意到这个数组是部分有序的
        // 我们设置lo和hi分别表示有限范围的两个端点
        // 这个地方我们要比较的是中间两个数字，mid和mid-1
        // 如果mid - 1大于mid，则说明最大值在左边，否则就在右边
        // AC 0ms 1.9mb
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2; // 为了保证不越界，hi - lo + 1
            if nums[mid - 1] > nums[mid] {
                hi = mid - 1;
            } else {
                lo = mid;
            }
        }
        lo as i32
    }
}