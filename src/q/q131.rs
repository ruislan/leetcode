use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 与86重名，改为palindrome_partition
    pub fn palindrome_partition(s: String) -> Vec<Vec<String>> {
        // 方法1
        // 回溯算法
        // 首先我们肯定需要一个函数来判断是不是palindrome
        // 然后我们来看字符串aab，我们知道要分割它的话，那么它的所有分割就是
        //      a         aa        aab
        //     a  ab      b
        //     b
        // 而我们需要将分割中符合标准的留下来，这样等于不合格的就不用继续判断下去了，
        // 很明显，符合标准就是所有的路径都是palindrome
        // 而我们要进入答案的标准就是所有的分割都是回文，那么也就是说我们的分割点要走到最后
        // 现在我们来按照我们的思想走一遍aab，
        // [a]
        //  [a]
        //   [b] -> [a,a,b]
        //  [ab]
        // [aa]
        //  [b] -> [aa, b]
        // [aab]
        // AC 132ms 20.1mb
        fn is_palindrome(s: &[u8], lo: usize, hi: usize) -> bool {
            let mut i = lo;
            let mut j = hi;
            while i < j {
                if s[i] != s[j] { return false; }
                i += 1;
                j -= 1;
            }
            true
        }

        fn dfs(arr: &mut Vec<String>, s: &str, i: usize, answer: &mut Vec<Vec<String>>) {
            if i == s.len() {
                answer.push(arr.clone());
                return;
            }
            for j in i..s.len() {
                if is_palindrome(s.as_bytes(), i, j) {
                    arr.push(s[i..=j].to_string());
                    dfs(arr, s, j + 1, answer);
                    arr.pop();
                }
            }
        }

        let mut answer = Vec::new();
        dfs(&mut Vec::new(), &s, 0, &mut answer);
        answer
    }
}