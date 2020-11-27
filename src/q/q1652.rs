use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // 首先记录code的长度n。
        // 由于数据量不是很大，所以我们可以将code进行复制追加
        // 这样我们就有两个code构成的数组，由于k在[-n+1, n - 1]之间，
        // 那么k的长度是不会超过单个code的
        // 创建answer[0;n]
        // 如果k == 0，直接answer
        // 如果k > 0， 那么我们从0..n，每次计算i+1..=i+k的距离，将和存入answer[i]
        // 如果k < 0， 那么我们从n..2n,每次计算i + k..i的距离（注意Rust的索引是usize），将和存入answer[i - n]
        // 返回answer
        // Passed 0ms 1.9mb
        let n = code.len();
        let mut code = code;
        let mut answer = vec![0; n];
        if k == 0 { return answer; }
        if k > 0 {
            let k = k as usize;
            code.extend(code.clone());
            for i in 0..n {
                let mut sum = 0;
                for j in (i + 1)..=(i + k) {
                    sum += code[j];
                }
                answer[i] = sum;
            }
        } else {
            let k = k.abs() as usize;
            code.extend(code.clone());
            for i in n..(n << 1) {
                let mut sum = 0;
                for j in (i - k)..i {
                    sum += code[j];
                }
                answer[i - n] = sum;
            }
        }
        answer
    }
}