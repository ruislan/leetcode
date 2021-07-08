use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // 方法1
        // hashmap
        // AC 0ms 2.3mb
        // let mut map = std::collections::HashMap::new();
        // let n = nums.len();
        // let half = n >> 1;
        // for i in 0..n {
        //     let count = map.entry(nums[i]).or_insert(0);
        //     *count += 1;
        //     if *count > half { return nums[i]; }
        // }
        // -1

        // 方法2
        // 摩尔投票
        // AC 0ms 2.3mb
        let n = nums.len();
        let mut answer = -1;
        let mut count = 0;
        for i in 0..n {
            if count == 0 { answer = nums[i]; }
            if nums[i] == answer { count += 1; } else { count -= 1; }
        }
        count = 0;
        for i in 0..n {
            if nums[i] == answer { count += 1; }
        }
        if count * 2 > n { answer } else { -1 }
    }
}