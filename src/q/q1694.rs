use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reformat_number(number: String) -> String {
        // 方法1
        // 首先清理掉所有的"-"和" "
        // 然后总长度为n
        // 如果n > 4， 分出1个3
        // 如果n == 4，分两个2
        // 如果n < 4， 分成1组
        // Passed 0ms 2.1mb
        let mut num = std::collections::VecDeque::new();
        for x in number.chars() {
            if x.is_ascii_digit() {
                num.push_back(x);
            }
        }
        let mut answer = String::new();
        while !num.is_empty() {
            if num.len() > 4 {
                for _ in 0..3 {
                    answer.push(num.pop_front().unwrap());
                }
                answer.push('-');
            } else if num.len() == 4 {
                for _ in 0..2 {
                    answer.push(num.pop_front().unwrap());
                }
                answer.push('-');
                for _ in 0..2 {
                    answer.push(num.pop_front().unwrap());
                }
            } else {
                answer.push(num.pop_front().unwrap());
            }
        }
        answer
    }
}