use crate::xh::Solution;

impl Solution {
    pub fn find_left_repeat_num_ii(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 用hashmap记录前一次出现的位置就行了
        // Passed 0ms 1.9mb
        let mut hashmap = std::collections::HashMap::new();
        let mut answer = Vec::new();
        for i in 0..nums.len() {
            if let Some(first_pos) = hashmap.get_mut(&nums[i]) {
                answer.push(*first_pos as i32);
                *first_pos = i;
            } else {
                hashmap.insert(nums[i], i);
                answer.push(-1);
            }
        }
        answer
    }
}