use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        // 方法1
        // 先统计各个字符的频率到freq
        // 然后扫描s
        // 如果s[i]不存在结果字符串中：
        //    循环比较结果字符串中的最后一个与s[i]：
        //        如果s[i]大于结果字符串的最后一个或者最后一个字符已经是频率中的最后一个了freq[prev] == 0
        //            不弹出
        //        否则：将最后一个字符从结果字符串中弹出
        // 返回结果字符串
        // Passed 0ms 2.1mb
        let mut freq = vec![0; 26];
        for x in s.bytes() {
            freq[x as usize - 97] += 1;
        }

        let mut answer = String::new();
        for x in s.chars() {
            if !answer.contains(x) {
                while let Some(prev) = answer.pop() {
                    if x as u8 > prev as u8 || freq[prev as u8 as usize - 97] == 0 {
                        answer.push(prev);
                        break;
                    }
                }
                answer.push(x);
            }
            freq[x as u8 as usize - 97] -= 1;
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_duplicate_letters("bcabc".to_string()), "abc".to_string());
    assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".to_string()), "acdb".to_string());
}