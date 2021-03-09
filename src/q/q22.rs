use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // 方法1
        // 回溯算法
        // 这题的回溯要转换一下，我们不要用完整的括号"()"去回溯
        // 也就是说选择列表应该是单个括号
        // 那么现在就是确定到底是用左还是右括号了
        // 我们肯定倾向于先用左括号，因为先用右括号是不合法的括号")("
        // 然后使用右括号，而右括号的使用有一个前提
        // 那么就是左括号使用的次数必须大于右括号，不然会有不合法的情况"())("
        // AC 0ms 2mb
        fn backtrace(arr: &mut String, l: usize, r: usize, len: usize, answer: &mut Vec<String>) {
            if arr.len() == len {
                answer.push(arr.clone());
                return;
            }
            if l > 0 {
                arr.push('(');
                backtrace(arr, l - 1, r, len, answer);
                arr.pop();
            }
            if l < r && r > 0 {
                arr.push(')');
                backtrace(arr, l, r - 1, len, answer);
                arr.pop();
            }
        }
        let mut answer = Vec::new();
        let n = n as usize;
        backtrace(&mut String::new(), n, n, n * 2, &mut answer);
        answer
    }
}