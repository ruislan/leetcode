use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        // 方法1
        // 这种括号配对的题还有计算器的题一看就是栈的应用
        // 这道题唯一的迷惑就是三种运算方式
        // ()得1分，而AB是A+B，(A)是2*A
        // 看这个式子很容易陷入误区，就是数字和()分开处理
        // 其实我们要抓住重点，重点就是这道题输出是数字
        // 我们需要的也是数字，所以栈应该存的是数字
        // 而)只是启动我们运算规则按钮。而(我们可以用数字0代替，因为得分不可能为负
        // 除非字符串非法，那么至少都有1分, S.len() >= 2
        // 所以当(的时候，我们就推0
        // 当)的时候，我们设置一个n来存储当前运算的结果
        // 然后循环判断栈顶，
        // 如果是(，那么如果有运算结果也就是n>0，就执行2*A这个运算，并退出
        //         如果没有运算过，也就是直接()的情况，执行n=1这个运算，并退出
        // 如果是数字，那就将数字取出来加入到n中，继续循环，直到(
        // 最后将结果n推入到栈
        // 当所有的字符都结束的时候
        // 我们栈内的数字全部加起来就是结果（因为有可能是()()这样平行的情况，所以还要加一次）
        // AC 0ms 2mb
        let mut stack = Vec::new();
        for ch in s.chars() {
            if ch == '(' {
                stack.push(0);
            } else {
                let mut n = 0;
                while let Some(x) = stack.pop() {
                    if x == 0 {
                        if n == 0 {
                            n = 1;
                        } else {
                            n *= 2;
                        }
                        break;
                    }
                    if x > 0 {
                        n += x;
                    }
                }
                stack.push(n);
            }
        }
        stack.into_iter().sum()
    }
}
