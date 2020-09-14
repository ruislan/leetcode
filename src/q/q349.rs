use crate::q::Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 方法1
        // let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        // let mut res: Vec<i32> = Vec::new();
        // for i in 0..nums1.len() {
        //     map.entry(nums1[i]).or_insert(0);
        // }
        // for i in 0..nums2.len() {
        //     let mut count = map.get_mut(&nums2[i]);
        //     if count != None {
        //         *count.unwrap() += 1;
        //     }
        // }
        // for (k, v) in map {
        //     if v >= 1 {
        //         res.push(k);
        //     }
        // }
        // res

        // 方法2
        let mut res: Vec<i32> = Vec::new();
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    if !res.contains(&nums1[i]) {
                        res.push(nums1[i]);
                    }
                }
            }
        }
        res
    }
}