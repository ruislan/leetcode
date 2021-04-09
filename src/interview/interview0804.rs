use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 回溯法解决
        // AC 0ms 2mb
        fn backtrace(path: &mut Vec<i32>, nums: &Vec<i32>, i: usize, answer: &mut Vec<Vec<i32>>) {
            answer.push(path.clone());
            for j in i..nums.len() {
                path.push(nums[j]);
                backtrace(path, nums, j + 1, answer);
                path.pop();
            }
        }
        let mut answer = Vec::new();
        backtrace(&mut Vec::new(), &nums, 0, &mut answer);
        answer
    }
}
