use crate::xh::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_left_repeat_num(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 用hashmap记录第一次出现的位置就行了
        // Passed 0ms 2.1mb
        let mut hashmap = std::collections::HashMap::new();
        let mut answer = Vec::new();
        for i in 0..nums.len() {
            if let Some(&first_pos) = hashmap.get(&nums[i]) {
                answer.push(first_pos as i32);
            } else {
                hashmap.insert(nums[i], i);
                answer.push(-1);
            }
        }
        answer
    }
}