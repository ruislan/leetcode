use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn majority_element_2(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 计数，O(n)空间，不是最佳
        // AC 0ms 2.3mb
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        let n = nums.len();
        let cap = n / 3;
        for x in nums {
            *freq.entry(x).or_insert(0) += 1;
        }
        freq.into_iter().filter(|(k, v)| *v > cap).map(|(k, v)| k).collect()
    }
}