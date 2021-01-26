use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        // 方法1
        // 直接模拟就行了
        // Passed 0ms 2.1mb
        if nums.len() < 2 { return nums.len() as i32; }
        let mut answer = 1;
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                count += 1;
            } else {
                count = 1;
            }
            answer = answer.max(count);
        }
        answer
    }
}