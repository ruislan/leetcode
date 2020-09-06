use crate::q::Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        // 方法1
        // 迭代s，检查s[i]的左右字符，从'a'-'z'中选择一个两边都不等的即可
        // Passed 0ms 2.1mb
        let mut res: Vec<char> = s.chars().collect();
        (0..s.len()).for_each(|i| {
            if res[i] == '?' {
                let left = if i == 0 { None } else { Some(res[i - 1]) };
                let right = if i == s.len() - 1 { None } else { Some(res[i + 1]) };
                res[i] = ('a' as u8..='z' as u8).find(|&x| Some(x as char) != left && Some(x as char) != right).unwrap() as char;
            }
        });
        res.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::modify_string("a".to_string()), "a".to_string());
    assert_eq!(Solution::modify_string("?".to_string()), "a".to_string());
    assert_eq!(Solution::modify_string("??".to_string()), "ab".to_string());
    assert_eq!(Solution::modify_string("???".to_string()), "aba".to_string());
    assert_eq!(Solution::modify_string("a?c".to_string()), "abc".to_string());
    assert_eq!(Solution::modify_string("?a?".to_string()), "bab".to_string());
    assert_eq!(Solution::modify_string("?zs".to_string()), "azs".to_string());
    assert_eq!(Solution::modify_string("ubv?w".to_string()), "ubvaw".to_string());
    assert_eq!(Solution::modify_string("j?qg??b".to_string()), "jaqgacb".to_string());
    assert_eq!(Solution::modify_string("??yw?ipkj?".to_string()), "abywaipkja".to_string());
    assert_eq!(Solution::modify_string("?".to_string().repeat(100)), "ab".to_string().repeat(50));
}