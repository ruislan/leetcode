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
        // AC 0ms 2mb
        fn combine(i: usize, n: usize, prev: &str, keyboard: &Vec<Vec<&str>>, digits: &Vec<u8>, arr: &mut Vec<String>) {
            if i >= n {
                arr.push(prev.to_string());
            } else {
                for key in keyboard[(digits[i] - b'1') as usize].iter() {
                    combine(i + 1, n, &(String::from(prev) + key), keyboard, digits, arr);
                }
            }
        }

        if digits.len() < 1 { return Vec::new(); }
        let digits = digits.into_bytes();
        let n = digits.len();
        let keyboard = vec![vec![], vec!["a", "b", "c"], vec!["d", "e", "f"],
                            vec!["g", "h", "i"], vec!["j", "k", "l"], vec!["m", "n", "o"],
                            vec!["p", "q", "r", "s"], vec!["t", "u", "v"], vec!["w", "x", "y", "z"]];
        let mut answer = Vec::new();
        combine(0, n, "", &keyboard, &digits, &mut answer);
        answer
    }
}