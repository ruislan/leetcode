use crate::q::Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // 方法1
        // for i in 0..nums.len() {
        //     for j in (i + 1)..nums.len() {
        //         if (j - i) > k as usize { break; }
        //         if nums[i] == nums[j] { return true; }
        //     }
        // }
        // false

        // 方法2
        let mut map = std::collections::HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];
            if let Some(value) = map.get(&num) {
                if i - *value <= k as usize { return true; }
            }
            map.insert(num, i);
        }
        false
    }
}