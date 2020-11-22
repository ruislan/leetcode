use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 方法1
        // let (m,n) = (m as usize, n as usize);
        // for i in m..(m+n) {
        //     nums1[i] = nums2[i - m];
        // }
        // nums1.sort();
        
        // 方法2
        if n == 0 { return (); }
        let (mut p1, mut p2, mut mv) = (m - 1, n - 1, 1);
        loop {
            if p1 >= 0 && nums1[p1 as usize] >= nums2[p2 as usize] {
                let temp = nums1[(m + n - mv) as usize];
                nums1[(m + n - mv) as usize] = nums1[p1 as usize];
                nums1[p1 as usize] = temp;
                p1 -= 1;
            } else {
                nums1[(m + n - mv) as usize] = nums2[p2 as usize];
                if p2 == 0 { break; }
                p2 -= 1;
            }
            mv += 1;
        }
    }
}