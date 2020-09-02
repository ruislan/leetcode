use crate::interview::Solution;

impl Solution {
    pub fn compress_string(s: String) -> String {
        // 方法1
        // 迭代s，如果与前一个相同，数量就增加1
        // 如果不相同就将数量压入字符串，然后将新的字符压入字符串
        // 并且将数量增加1
        // 循环结束，将最后这个字符的数量压入字符串
        // 如果字符串长度大于等于原字符串，返回原字符串，否则返回新字符串
        // 0ms 2.4mb
        // let mut res = String::new();
        // let mut chars: Vec<char> = s.chars().collect();
        // let mut ptr = 0;
        // for i in 1..chars.len() {
        //     if chars[i] != chars[i - 1] {
        //         res.push(chars[i - 1]);
        //         res.push_str(&(i - ptr).to_string());
        //         ptr = i;
        //     }
        // }
        // if ptr < s.len() {
        //     res.push(chars[s.len() - 1]);
        //     res.push_str(&(s.len() - ptr).to_string());
        // }
        // if res.len() < s.len() { res } else { s }

        // 方法2
        // 方法1的改进，方法1中还需要转换成Vec<char>有点啰嗦了
        // 0ms 2mb
        let mut res = String::new();
        let (mut last, mut count) = ('0', 0);
        for ch in s.chars() {
            if ch != last {
                if count > 0 {
                    res.push(last);
                    res.push_str(&count.to_string());
                }
                last = ch;
                count = 0;
            }
            count += 1
        }
        if count > 0 {
            res.push(last);
            res.push_str(&count.to_string());
        }
        if res.len() < s.len() { res } else { s }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::compress_string("".to_string()), "".to_string());
    assert_eq!(Solution::compress_string("a".to_string()), "a".to_string());
    assert_eq!(Solution::compress_string("aa".to_string()), "aa".to_string());
    assert_eq!(Solution::compress_string("aaa".to_string()), "a3".to_string());
    assert_eq!(Solution::compress_string("aaabbbccc".to_string()), "a3b3c3".to_string());
}
