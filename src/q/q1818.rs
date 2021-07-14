use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // 方法1
        // 两两比较，找出nums1[i] - nums2[i] 与nums1[j] - nums2[i] ，i in [0..n)且i!=j的最大值
        // 这就是能够贡献最大的
        // O(n^2)
        // 超时
        // let n = nums1.len();
        // let mut max_diff = 0;
        // let mut replace = 0;
        // let mut pos = 0;
        // for i in 0..n {
        //     let abs = (nums1[i] - nums2[i]).abs();
        //     for j in 0..n {
        //         if i == j { continue; }
        //         let abs1 = (nums1[j] - nums2[i]).abs();
        //         if abs1 >= abs { continue; }
        //
        //         let diff = (abs - abs1).abs();
        //         if diff > max_diff {
        //             max_diff = diff;
        //             replace = abs1;
        //             pos = i;
        //         }
        //     }
        // }
        // let mut answer = 0;
        // for i in 0..n {
        //     let abs = if i == pos { replace } else { (nums1[i] - nums2[i]).abs() };
        //     answer += abs;
        //     answer %= 1000000007;
        // }
        // answer


        // 方法2
        // 我们需要优化方法1
        // 假设我们在查找上面abs1的时候能够快速找到就好了
        // 因为我们要的是确切的那个数字，所以我们可以排序nums1，
        // 然后二分查找找出最接近的答案，如果那个答案的下标与当前这个相同，则比较左右两个即可。
        // AC 40ms 3.3mb
        let n = nums1.len();
        let mut nums = nums1.clone();
        nums.sort_unstable();

        let mut max = 0_i64;
        let mut sum = 0_i64;
        for i in 0..n {
            let a = nums1[i];
            let b = nums2[i];
            if a == b { continue; }
            let abs = (a - b).abs() as i64;
            sum += abs;
            let mut lo = 0;
            let mut hi = n - 1;
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if nums[mid] <= b {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            let mut x = (nums[hi] - b).abs() as i64;
            if hi + 1 < n { x = x.min((nums[hi + 1] - b).abs() as i64); }
            if x < abs { max = max.max((abs - x) as i64); }
        }
        ((sum - max) % 1000000007) as i32
    }
}