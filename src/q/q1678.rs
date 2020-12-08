use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn interpret(command: String) -> String {
        // 方法1
        // 逐步检查command
        //   如果是'G'，拼接G
        //   否则（也就是'('）：检查下一个字符是什么：
        //      如果是')'，拼接o
        //      否则（也就是"(al)"），拼接al，同时让再跳过两个字符(也就是跳过"l)")
        // 因为输入范围只有G、()、(al)这三个，所以不必考虑其他情况
        // 直到最后一个字符，返回拼接结果
        // 0ms 1.9mb
        let mut iter = command.chars();
        let mut answer = String::new();
        while let Some(ch) = iter.next() {
            if ch == 'G' {
                answer.push(ch);
            } else {
                if let Some(next) = iter.next() {
                    if next == ')' {
                        answer.push('o');
                    } else {
                        answer.push_str("al");
                        iter.next();
                        iter.next();
                    }
                }
            }
        }
        answer
    }
}