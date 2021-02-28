use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 回溯搞定
        // 还有很多优化空间哈，这个只是最基本的
        // Passed 204ms 2mb
        fn backtrace(path: &mut Vec<i32>, candidates: &Vec<i32>, idx: usize, sum: i32, target: i32, answer: &mut std::collections::HashSet<Vec<i32>>) {
            if sum == target {
                answer.insert(path.clone());
                return;
            }
            for i in idx..candidates.len() {
                if sum + candidates[i] <= target {
                    path.push(candidates[i]);
                    backtrace(path, candidates, i + 1, sum + candidates[i], target, answer);
                    path.pop();
                }
            }
        }
        let mut answer = std::collections::HashSet::new();
        let mut candidates = candidates;
        candidates.sort_unstable();
        backtrace(&mut Vec::new(), &candidates, 0, 0, target, &mut answer);
        answer.into_iter().collect()
    }
}