use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        // 方法1
        // 利用异或，将两个相同的两两相消
        // 时间：O(n)，空间O(1)
        // AC 0ms 2mb
        // nums.into_iter().fold(0, |acc, x| acc ^ x)
        
        // 方法2
        // 其实本题的要求是时间：O(logn)，空间:O(1)
        // 根据题目给出的条件是有序数组，结合O(logn)
        // 我们就知道只能二分啦
        // 这里我们可以观察到，其他都是两个，只有一个是单个，那么数组总数必然是奇数
        // 二分的索引位置如果是奇数，那么实际上这个数字在偶数位置上
        //     检查它的左边和右边，
        //     if:左右都不相同，那么就找到了
        //     else if:左边和它相同，则说明这半边都是成双的，没有单个，则向右查找
        //     else:右边和它相同，则说明右边都是成双的， 则向左查找
        // 索引位置如果是偶数， 那么实际上这个数字在奇数位置上
        //     检查它的左右和右边，
        //     if:左右都不相同，那么就找到了
        //     else if：左边和它相同，则说明单个的在左边，则向左查找
        //     else: 右边和它相同，则说明单个的在右，则向右查找
        // AC 0ms 2mb
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        while lo <= hi  {
            let mid = lo + (hi - lo) / 2;
            let is_mid_at_even = mid & 1 == 1;
            let mid_value = nums[mid];
            let left_value = if mid == 0 { mid_value - 1 } else { nums[mid - 1] };
            let right_value = if mid == n - 1 { mid_value - 1} else { nums[mid + 1] };
            if mid_value != left_value && mid_value != right_value { return mid_value; }
            if is_mid_at_even {
                if left_value == mid_value {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            } else {
                if left_value == mid_value {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }
        }
        -1
    }
}