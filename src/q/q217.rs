use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        // 方法1
        // 暴力法，每个数字向后面比对一次
        // 时间O(n^2)，空间O(1)
        // Passed 672ms 2.6mb
        // for i in 0..nums.len() {
        //     let n1 = nums[i];
        //     for j in (i + 1)..nums.len() {
        //         let n2 = nums[j];
        //         if n1 == n2 { return true; }
        //     }
        // }
        // false

        // 方法2
        // 排序后相同的数字自然在一起，检查相邻数字是否相同即可
        // 时间O(nlogn + n)，空间O(1)
        // Passed 0ms 2.6mb
        // nums.sort();
        // for i in 1..nums.len() {
        //     if nums[i] == nums[i - 1] { return true; }
        // }
        // false

        // 方法3
        // 用hashset存储，不能存储的说明已经有相同的了
        // 时间O(n)，空间O(n)，
        // 即便如此时间上也不如方法2，
        // 因为Rust的排序很快，hash的时间反而比较重
        // Passed 4ms 2.6mb
        let mut sets = std::collections::HashSet::new();
        for num in nums {
            if !sets.insert(num) {
                return true;
            }
        }
        false
    }
}