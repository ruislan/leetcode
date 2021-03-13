use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 回溯法，套模板就行了
        // 这里由于每个情况都是结果，所以每个情况都要保存一下
        // 然后就是退出条件就是选择列表没有可以再有可选的即可
        // 由于输入的数据有重复的，所以产生的结果有可能重复，
        // 简单点直接采用hashset来解决就行了
        // AC 0ms 2.1mb
        use std::collections::HashSet;
        fn backtrace(arr: &mut Vec<i32>, nums: &Vec<i32>, i: usize, answer: &mut HashSet<Vec<i32>>) {
            answer.insert(arr.clone());
            for j in i..nums.len() {
                arr.push(nums[j]);
                backtrace(arr, nums, j + 1, answer);
                arr.pop();
            }
        }
        let mut answer = HashSet::new();
        let mut nums = nums;
        nums.sort_unstable();
        backtrace(&mut Vec::new(), &nums, 0, &mut answer);
        answer.into_iter().collect()
    }
}