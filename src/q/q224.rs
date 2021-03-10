use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        // 方法1
        // 看到括号就进行将括号内部的数据先进行计算
        // 例如: 1 + (10-9)就先计算10-9
        // 利用栈存储这些字符，当出现数字的时候，我们计算数字，并将前面的符号带上进行加减即可
        // 注意数字有可能是多位的，所以这里我们需要一个函数来解决
        // 计算完成后的数字，我们用'x'这个字符来替代，这样当遇到'x'字符的时候，就知道是要取一个中间结果了
        // 而我们把中间结果都放到一个栈里面解决
        // AC 0ms 3.3mb
        // fn get_num(f: &mut i32, nums: &mut Vec<i32>) -> i32 {
        //     if *f > 0 {
        //         let mut num = 0;
        //         while *f > 0 {
        //             num = num * 10 + nums.pop().unwrap();
        //             *f -= 1;
        //         }
        //         num
        //     } else if let Some(num) = nums.pop() {
        //         num
        //     } else {
        //         0
        //     }
        // }
        //
        // let mut que = Vec::new();
        // let mut answer = Vec::new();
        // let mut chars: Vec<char> = s.chars().filter(|&x| x != ' ').map(|x| x).collect();
        // let n = chars.len();
        // for i in 0..=n {
        //     if i < n && chars[i] != ')' {
        //         que.push(chars[i]);
        //         continue;
        //     }
        //     let mut nums = Vec::new();
        //     let mut sum = 0;
        //     let mut f = 0;
        //     while let Some(ch) = que.pop() {
        //         match ch {
        //             '0'..='9' => {
        //                 nums.push(ch as i32 - '0' as i32);
        //                 f += 1;
        //             }
        //             'x' => nums.push(answer.pop().unwrap()),
        //             '+' => sum += get_num(&mut f, &mut nums),
        //             '-' => sum -= get_num(&mut f, &mut nums),
        //             _ => break,
        //         }
        //     }
        //     sum += get_num(&mut f, &mut nums);
        //     answer.push(sum);
        //     que.push('x');
        // }
        // answer.pop().unwrap()

        // 方法2
        // 方法1中我们知道我们是碰到")"之后再回过头去算
        // 其实完全没有这个必要，我们可以顺着迭代，直接算出数字
        // 这样就只需要解决一个符号的问题，我们可以把符号设置成1和-1，这样正负数可以解决，
        // 而符号的问题有2种
        // 1，在[数字]前面，那么我们的answer += 符号 * [数字]，
        //    这里的[数字]有可能是单纯的数字也有可能一个表达式"(...)"的结果数字，也就是说符号可能在"("前，
        // 2，在"("里面，那么"("里面的数字的符号就变成相反的方向，要处理这个问题，
        //    我们只需要保持“(”前面的符号，而这个完全可以用 前面的符号*当前符号即可，
        //    假设前面是"-"号，而括号里面是"+"号，-1 * 1 = -1，而这正好是我们想要的
        //    假设前面是“-”号，而括号里面是"-"号，-1 * -1 = 1，而这正好是我们想要的
        // 所以我们需要一个栈来存储括号前的符号，然后当遇到“)”的时候，表示我们处理完了，不需要了，就pop掉。
        // AC 0ms 3mb
        let mut ops = vec![1];
        let mut op = 1;
        let mut answer = 0;
        let mut chars: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut i = 0;
        while i < n {
            match chars[i] {
                ' ' => i += 1,
                '+' => {
                    op = *ops.last().unwrap();
                    i += 1;
                }
                '-' => {
                    op = -*ops.last().unwrap();
                    i += 1;
                }
                '(' => {
                    ops.push(op);
                    i += 1;
                }
                ')' => {
                    ops.pop();
                    i += 1;
                }
                '0'..='9' => {
                    let mut num = 0;
                    while i < n && chars[i].is_ascii_digit() {
                        num = num * 10 + chars[i] as i32 - '0' as i32;
                        i += 1;
                    }
                    answer += op * num;
                }
                _ => ()
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::calculate("-10 + (-200+ 1)".to_string()), -209);
}