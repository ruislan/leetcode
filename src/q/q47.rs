use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 全排列看到第一想到的就是回溯
        // 标准模板套用之后，关键就是筛出合适的选择
        use std::collections::{HashSet, HashMap};
        fn backtrace(path: &mut Vec<i32>, nums: &Vec<i32>, used: &mut Vec<i32>, answer: &mut HashSet<Vec<i32>>) {
            if path.len() == nums.len() {
                answer.insert(path.clone());
                return;
            }
            for &x in nums {
                if used[(x + 10) as usize] > 0 {
                    used[(x + 10) as usize] -= 1;
                    path.push(x);
                    backtrace(path, nums, used, answer);
                    path.pop();
                    used[(x + 10) as usize] += 1;
                }
            }
        }

        let mut answer = HashSet::new();
        let mut freq = vec![0; 21];
        for x in nums.iter() { freq[(x + 10) as usize] += 1; }
        backtrace(&mut Vec::new(), &nums, &mut freq, &mut answer);
        answer.into_iter().collect()
    }
}