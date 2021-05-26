use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        // 方法1
        // 看着这种括号就是用栈来解决了
        // 我们将字符串转换成字符数组，这样可以通过指针来操作
        // 我们用栈来记录'('的索引位置，如果遇到')'，则移除栈顶
        // 这样我们可以得到一组()的起止位置，然后直接反转这两个位置的字符
        // 重复上述操作，直到结束
        // 然后我们遍历这个字符数组，剔除掉'('和')'就是结果了
        // 例如: a(u(love)i)
        //      a(u)evol(i)   反转了(love)
        //      a)i(love)u(   反转了(u)evol(i)
        //      ailoveu       剔除了()
        // AC 0ms 2.1mb
        let n = s.len();
        let mut chars: Vec<char> = s.chars().collect();
        let mut stack = Vec::new();

        for i in 0..n {
            if chars[i] == '(' {
                stack.push(i);
            } else if chars[i] == ')' {
                let start = stack.pop().unwrap();
                let end = i;
                for j in start..((start + end) / 2 + 1) {
                    chars.swap(j, end - j + start);
                }
            }
        }
        let mut answer = String::new();
        for i in 0..n {
            if chars[i] != '(' && chars[i] != ')' {
                answer.push(chars[i]);
            }
        }
        answer
    }
}