use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        // 方法1
        // 统计字符的频率freq
        // 按照频率从高到低排列
        // 取最多的那个隔一个填充到结果数组，取完一组取下一组，直到全部完成
        // 将数组组合成字符串返回
        // Passed 0ms 2mb
        let n = s.len();
        if n < 2 {
            return s;
        }

        let mut freq = vec![0; 26];
        s.into_bytes().into_iter().for_each(|x| freq[(x - 97) as usize] += 1);

        let mut freq: Vec<Vec<u8>> = freq.into_iter().enumerate().filter(|&x| x.1 > 0).map(|x| vec![(x.0 + 97) as u8; x.1]).collect();
        freq.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

        let mut bytes: Vec<u8> = freq.into_iter().flatten().collect();
        let mut answer = vec![0; n];
        (0..n).step_by(2).for_each(|i| answer[i] = bytes.pop().unwrap());
        (1..n).step_by(2).for_each(|i| answer[i] = bytes.pop().unwrap());

        if (1..n).any(|i| answer[i] == answer[i - 1]) { return String::new(); }
        answer.into_iter().map(|x| x as char).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reorganize_string("a".to_string()), "a".to_string());
    assert_eq!(Solution::reorganize_string("aa".to_string()), "".to_string());
    assert_eq!(Solution::reorganize_string("ab".to_string()), "ba".to_string());
    assert_eq!(Solution::reorganize_string("aab".to_string()), "aba".to_string());
    assert_eq!(Solution::reorganize_string("abb".to_string()), "bab".to_string());
    assert_eq!(Solution::reorganize_string("abbb".to_string()), "".to_string());
}