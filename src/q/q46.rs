use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 回溯算法
        // 设置一个arr来跟踪回溯的路径，used来跟踪可供的选择
        // 当路径达到nums.len()的时候，就是答案之一
        // 例如 1 2 3
        // 最开始我们的arr是[]，可供选择是1|2|3
        //    选择1之后，[1]，可供选择是2|3
        //       选择2之后，[1,2]，可供选择只有3
        //          选择3之后，[1,2,3]进入答案
        //       选择3之后，[1,3]，可供选择只有2
        //          选择2之后，[1,3,2]进入答案
        //    选择2之后，[2]，可供选择是1|3
        //       选择1之后，[2,1]，可供选择只有3
        //       ...
        //    选择3之后，[3]，可供选择是1|2
        //       ...
        // AC 0ms 2.1mb
        fn backtrace(arr: &mut Vec<i32>, nums: &Vec<i32>, used: &mut std::collections::HashSet<i32>, answer: &mut Vec<Vec<i32>>) {
            if arr.len() == nums.len() {
                answer.push(arr.clone());
            } else {
                for &x in nums {
                    if used.insert(x) {
                        arr.push(x);
                        backtrace(arr, nums, used, answer);
                        arr.pop();
                        used.remove(&x);
                    }
                }
            }
        }

        let mut answer = Vec::new();
        backtrace(&mut Vec::new(), &nums, &mut std::collections::HashSet::new(), &mut answer);
        answer
    }
}