use crate::q::Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 方法1
        // Passed 0ms 2.1mb
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
        // Passed 0ms 1.9mb
        // let mut res: Vec<i32> = Vec::new();
        // for i in 0..nums1.len() {
        //     for j in 0..nums2.len() {
        //         if nums1[i] == nums2[j] {
        //             if !res.contains(&nums1[i]) {
        //                 res.push(nums1[i]);
        //             }
        //         }
        //     }
        // }
        // res

        // 方法3
        // 利用hashset搞定
        // Passed 0ms 2mb
        let mut sets = std::collections::HashSet::new();
        let mut answer = std::collections::HashSet::new();
        nums1.into_iter().for_each(|n| { sets.insert(n); });
        nums2.into_iter().for_each(|n| if sets.contains(&n) { answer.insert(n); });
        answer.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::intersection(vec![], vec![]), vec![]);
    assert_eq!(Solution::intersection(vec![1], vec![1]), vec![1]);
    assert_eq!(Solution::intersection(vec![1], vec![]), vec![]);
    assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    assert_eq!(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![9, 4]);
}