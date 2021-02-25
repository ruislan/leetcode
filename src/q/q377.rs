use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // 方法1
        // 回溯算法搞定组合排列全家桶
        // 基本和它前几个系列一样，没啥新东西
        // 简单就是上模板，注意一下出口和进入结果集的条件,done
        fn backtrace(path: &mut Vec<i32>, nums: &Vec<i32>, sum: i32, target: i32, answer: &mut i32) {
            if sum == target {
                *answer += 1;
                return;
            }
            for i in 0..nums.len() {
                if nums[i] + sum <= target {
                    path.push(nums[i]);
                    backtrace(path, nums, nums[i] + sum, target, answer);
                    path.pop();
                }
            }
        }
        let mut answer = 0;
        backtrace(&mut Vec::new(), &nums, 0, target, &mut answer);
        answer
    }
}