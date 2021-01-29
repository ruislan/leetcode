use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // 方法1
        // 每个数字都包含一些字母
        // 如果要组合起来，那么必然要让各个数字配对
        // 这样一看回溯法就是一个比较适合的方法
        // 回溯与递归是很搭配的
        // 那么递归的出口就是我们的组合到了最后一个数字
        // 也即是长度到达了digits.len()
        // 那么就可以将结果放入结果集合了
        // 中间过程只需要我们之前的组合prev，
        // 然后把之前的组合prev和当前这个数字的字母集一一递归配对即可
        // 题目提到“给定一个仅包含2-9的字符串”，所以这里我就不去判断2-9以外的数字了
        fn combine(i: usize, n: usize, prev: String, digits: &Vec<u8>, arr: &mut Vec<String>) {
            if i >= n {
                arr.push(prev);
            } else {
                let digit = digits[i];
                match digit {
                    b'2' => {
                        combine(i + 1, n, prev.clone() + "a", digits, arr);
                        combine(i + 1, n, prev.clone() + "b", digits, arr);
                        combine(i + 1, n, prev.clone() + "c", digits, arr);
                    }
                    b'3' => {
                        combine(i + 1, n, prev.clone() + "d", digits, arr);
                        combine(i + 1, n, prev.clone() + "e", digits, arr);
                        combine(i + 1, n, prev.clone() + "f", digits, arr);
                    }
                    b'4' => {
                        combine(i + 1, n, prev.clone() + "g", digits, arr);
                        combine(i + 1, n, prev.clone() + "h", digits, arr);
                        combine(i + 1, n, prev.clone() + "i", digits, arr);
                    }
                    b'5' => {
                        combine(i + 1, n, prev.clone() + "j", digits, arr);
                        combine(i + 1, n, prev.clone() + "k", digits, arr);
                        combine(i + 1, n, prev.clone() + "l", digits, arr);
                    }
                    b'6' => {
                        combine(i + 1, n, prev.clone() + "m", digits, arr);
                        combine(i + 1, n, prev.clone() + "n", digits, arr);
                        combine(i + 1, n, prev.clone() + "o", digits, arr);
                    }
                    b'7' => {
                        combine(i + 1, n, prev.clone() + "p", digits, arr);
                        combine(i + 1, n, prev.clone() + "q", digits, arr);
                        combine(i + 1, n, prev.clone() + "r", digits, arr);
                        combine(i + 1, n, prev.clone() + "s", digits, arr);
                    }
                    b'8' => {
                        combine(i + 1, n, prev.clone() + "t", digits, arr);
                        combine(i + 1, n, prev.clone() + "u", digits, arr);
                        combine(i + 1, n, prev.clone() + "v", digits, arr);
                    }
                    _ => {
                        combine(i + 1, n, prev.clone() + "w", digits, arr);
                        combine(i + 1, n, prev.clone() + "x", digits, arr);
                        combine(i + 1, n, prev.clone() + "y", digits, arr);
                        combine(i + 1, n, prev.clone() + "z", digits, arr);
                    }
                }
            }
        }
        if digits.len() < 1 { return Vec::new(); }
        let digits = digits.into_bytes();
        let n = digits.len();
        let mut answer = Vec::new();
        combine(0, n, String::new(), &digits, &mut answer);
        answer
    }
}