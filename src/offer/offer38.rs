use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        // 方法1
        // 回溯算法
        // AC 132ms 4mb
        use std::collections::HashSet;
        fn backtrace(path: &mut Vec<char>, used: &mut HashSet<usize>, s: &Vec<char>, answer: &mut HashSet<String>) {
            if used.len() == s.len() {
                answer.insert(path.iter().collect());
                return;
            }
            for i in 0..s.len() {
                if used.insert(i) {
                    path.push(s[i]);
                    backtrace(path, used, s, answer);
                    path.pop();
                    used.remove(&i);
                }
            }
        }

        let mut answer = HashSet::new();
        backtrace(&mut Vec::new(), &mut HashSet::new(), &s.chars().collect(), &mut answer);
        answer.into_iter().collect()
    }
}