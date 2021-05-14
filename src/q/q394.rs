use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn decode_string(s: String) -> String {
        // 方法1
        // 通过递归来处理每一个digit[..]
        // 遇到数字，就将后续的[]选出来放入递归中处理
        // 处理完成后只需要将结果重复数字的次数即可
        // AC 0ms 2.1mb
        fn decode(s: &Vec<char>, start: usize, end: usize) -> String {
            let mut answer = String::new();
            let mut i = start;
            let mut num = 0;
            while i < end {
                if s[i].is_ascii_digit() {
                    num = 10 * num + s[i].to_digit(10).unwrap();
                    i += 1;
                } else if s[i] == '[' {
                    let mut close = 1;
                    let mut j = i + 1;
                    while j < end {
                        if s[j] == '[' { close += 1; }
                        if s[j] == ']' { close -= 1; }
                        if close == 0 { break; }
                        j += 1;
                    }
                    answer += &decode(s, i + 1, j).repeat(num as usize);
                    i = j + 1;
                    num = 0;
                } else {
                    answer.push(s[i]);
                    i += 1;
                }
            }
            answer
        }
        let n = s.len();
        decode(&s.chars().collect(), 0, n)

        // 方法2
        // 双栈，数字存放数字栈，字符存放字符栈，
        // 遇到']'就弹出直到'['，然数字栈弹出一个数字
        // 后面来写代码
    }
}