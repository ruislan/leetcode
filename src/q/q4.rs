use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 方法1
        // 拼接数组，然后排序，然后找出中位数
        // O(nlogn)
        // AC 8ms 2mb
        // let mut nums1 = nums1;
        // let mut nums2 = nums2;
        // nums1.append(&mut nums2);
        // nums1.sort_unstable();
        // let m = nums1.len() / 2;
        // if nums1.len() % 2 == 0 {
        //     (nums1[m] + nums1[m - 1]) as f64 / 2.0
        // } else {
        //     nums1[m] as f64
        // }

        // 方法2
        // 归并有序数组
        // O(m + n)
        // AC 4ms 1.9mb
        let n = nums1.len();
        let m = nums2.len();
        let mut i = 0;
        let mut j = 0;
        let mut nums = vec![0; n + m];
        let l = nums.len();
        for k in 0..l {
            if i == n {
                nums[k] = nums2[j];
                j += 1;
            } else if j == m {
                nums[k] = nums1[i];
                i += 1;
            } else if nums1[i] > nums2[j] {
                nums[k] = nums2[j];
                j += 1;
            } else {
                nums[k] = nums1[i];
                i += 1;
            }
        }
        let mid = l / 2;
        if l & 1 == 1 {
            nums[mid] as f64
        } else {
            (nums[mid] + nums[mid - 1]) as f64 / 2.0
        }
    }
}
