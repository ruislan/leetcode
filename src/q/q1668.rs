use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        // 方法1
        // 令s = word
        // 如果sequence包含s，那么就将word加入到s中，重复判断，直到不包含为止
        // 返回包含的统计次数
        // Passed 0ms 2mb
        let mut answer = 0;
        let mut s = word.clone();
        while sequence.contains(&s) {
            s.push_str(&word);
            answer += 1;
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_repeating("ababc".to_string(), "ab".to_string()), 2);
}