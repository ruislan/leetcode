use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        // 方法1
        // 就是标准的回溯算法
        // 套用回溯模板就行了
        // 不用管元音，就看成5个数字就行
        // 这个题如果数学和概率学很熟悉的话，应该会有更简单的数学方法解决
        // AC 116ms 1.9mb
        // fn backtrace(path: &mut Vec<i32>, n: usize, i: i32, answer: &mut i32) {
        //     if path.len() == n {
        //         *answer += 1;
        //         return;
        //     }
        //     for x in i..5 {
        //         path.push(x);
        //         backtrace(path, n, x, answer);
        //         path.pop();
        //     }
        // }
        // let mut answer = 0;
        // backtrace(&mut Vec::new(), n as usize, 0, &mut answer);
        // answer

        // 方法2
        // 数学方法
        // C(4, n+4)
        // (n+4)*..*(n+1) / (4*..*1)
        // AC 0ms 2.1mb
        (n + 4) * (n + 3) * (n + 2) * (n + 1) / 24
    }
}