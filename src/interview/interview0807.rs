use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        // 方法1
        // 排列组合回溯法，套回溯的模板即可。
        // AC 52ms 4.6mb
        use std::collections::HashSet;
        fn backtrace(path: &mut Vec<char>, used: &mut HashSet<char>, s: &Vec<char>, answer: &mut Vec<String>) {
            if path.len() == s.len() {
                answer.push(path.clone().iter().collect());
                return;
            }
            for i in 0..s.len() {
                if !used.contains(&s[i]) {
                    path.push(s[i]);
                    used.insert(s[i]);
                    backtrace(path, used, s, answer);
                    path.pop();
                    used.remove(&s[i]);
                }
            }
        }
        let mut answer = Vec::new();
        backtrace(&mut Vec::new(), &mut HashSet::new(), &s.chars().collect(), &mut answer);
        answer
    }
}