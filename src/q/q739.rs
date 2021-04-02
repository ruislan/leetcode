use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 基本的单调栈解决即可
        // AC 24ms 2.7mb
        let n = t.len();
        let mut answer = vec![0; n];
        let mut stack:Vec<(usize, i32)> = Vec::new();
        for i in 0..n {
            while !stack.is_empty() && t[i] > stack.last().unwrap().1 {
                let last = stack.pop().unwrap();
                answer[last.0] = (i - last.0) as i32;
            }
            stack.push((i, t[i]));
        }
        answer
    }
}