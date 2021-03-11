use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法与q224重名，更名为calculate_ii
    pub fn calculate_ii(s: String) -> i32 {
        // 方法1
        // 这个题是q224的简化版本，虽然多了乘除，但是少了括号，同时还少了负数
        // 做法有很多，不过基本核心都是栈，因为要存储一个中间数据
        // 这里我延续了q226的思路，采用了双栈的思想，将数字和操作符分别存储起来
        // 遇到加减先不计算，遇到乘除就计算，然后将结果存储回数字栈
        // 这样如果最后剩下了操作符那么一定是加减，这样我们可以顺序的处理加减法
        // AC 0ms 3.8mb
        // let mut ops = Vec::new();
        // let mut nums = std::collections::VecDeque::new();
        // let chars: Vec<char> = s.chars().collect();
        // let n = chars.len();
        // let mut i = 0;
        // while i < n {
        //     match chars[i] {
        //         ' ' => i += 1,
        //         '0'..='9' => {
        //             let mut x = 0;
        //             while i < n && chars[i].is_ascii_digit() {
        //                 x = x * 10 + chars[i] as i32 - '0' as i32;
        //                 i += 1;
        //             }
        //             let y = if let Some(&op) = ops.last() {
        //                 match op {
        //                     '*' => {
        //                         ops.pop();
        //                         nums.pop_back().unwrap() * x
        //                     }
        //                     '/' => {
        //                         ops.pop();
        //                         nums.pop_back().unwrap() / x
        //                     }
        //                     _ => x,
        //                 }
        //             } else {
        //                 x
        //             };
        //             nums.push_back(y);
        //         }
        //         _ => {
        //             ops.push(chars[i]);
        //             i += 1;
        //         }
        //     }
        // }
        // let mut answer = nums.pop_front().unwrap();
        // for op in ops {
        //     answer += if op == '+' { 1 } else { -1 } * nums.pop_front().unwrap();
        // }
        // answer

        // 方法2
        // 单栈法，我们的栈只存储数字，然后用一个变量存储前一个操作符，
        // 这样，遇到乘除的时候先处理，遇到加减就把数字带正负号（前一个操作符）放入即可
        // AC 0ms 2.8mb
        let n = s.len();
        let mut nums = Vec::new();
        let mut x = 0;
        let mut pre_sign = '+';
        for (i, ch) in s.chars().enumerate() {
            if ch == ' ' { continue; }
            if ch.is_ascii_digit() {
                x = x * 10 + ch as i32 - '0' as i32;
                if i < n - 1 { continue; }
            }
            match pre_sign {
                '+' => nums.push(x),
                '-' => nums.push(-x),
                '*' => {
                    let y = nums.pop().unwrap() * x;
                    nums.push(y);
                }
                _ => {
                    let y = nums.pop().unwrap() / x;
                    nums.push(y);
                }
            }
            x = 0;
            pre_sign = ch;
        }
        nums.into_iter().sum()
    }
}