use crate::q::Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        // 方法1
        // let mut res: Vec<i32> = Vec::new();
        // nums1.sort();
        // nums2.sort();
        // let (mut p1, mut p2) = (0, 0);
        // loop {
        //     if p1 >= nums1.len() || p2 >= nums2.len() { break; }
        //     if nums1[p1] > nums2[p2] { p2 += 1; } else if nums1[p1] < nums2[p2] { p1 += 1; } else {
        //         res.push(nums1[p1]);
        //         p1 += 1;
        //         p2 += 1;
        //     }
        // }
        // res
        
        // 方法2
        let mut res: Vec<i32> = Vec::new();
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    res.push(nums1[i]);
                    nums2.remove(j);
                    break;
                }
            }
        }
        res
    }
}