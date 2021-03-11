use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        // 方法1
        // 解类似表达式、解析器什么的，栈就是核心。
        // 逆波兰表示法就是将算术符号后缀的表达法，例如：1,+,2就变成了1,2,+
        // 这样我们其实看得出来规律就是只要遇到符号，那么就把前两个数字拿出来处理
        // 处理完成之后再推回去，直到最后只剩下一个数字，就是结果。
        // AC 0ms 2.6mb
        let mut nums = Vec::new();
        for token in tokens {
            if token == "+" {
                let b = nums.pop().unwrap();
                let a = nums.pop().unwrap();
                nums.push(a + b);
            } else if token == "-" {
                let b = nums.pop().unwrap();
                let a = nums.pop().unwrap();
                nums.push(a - b);
            } else if token == "*" {
                let b = nums.pop().unwrap();
                let a = nums.pop().unwrap();
                nums.push(a * b);
            } else if token == "/" {
                let b = nums.pop().unwrap();
                let a = nums.pop().unwrap();
                nums.push(a / b);
            } else {
                let x = token.parse::<i32>().unwrap();
                nums.push(x);
            }
        }
        nums[0]
    }
}