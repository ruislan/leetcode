use crate::q::Solution;

impl Solution {
    // 给定两个由小写字母构成的字符串 A 和 B ，只要我们可以通过交换 A 中的两个字母得到与 B 相等的结果，就返回 true ；否则返回 false 。
    // 0 <= A.length <= 20000
    // 0 <= B.length <= 20000
    // A 和 B 仅由小写字母构成。
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut buckets = vec![0; 256];
        let a = a.chars().collect::<Vec<char>>();
        let b = b.chars().collect::<Vec<char>>();
        let mut swap = (0, 0);
        let mut count = 0;
        for i in 0..a.len() {
            let ch1 = a[i] as u8;
            let ch2 = b[i] as u8;
            buckets[ch1 as usize] += 1;

            if ch1 != ch2 {
                if count > 2 {
                    return false;
                }
                if count == 0 {
                    swap.0 = ch1;
                    swap.1 = ch2;
                } else if swap.0 != ch2 || swap.1 != ch1 {
                    return false;
                }
                count += 1;
            }
        }
        if count == 0 {
            for i in 0..256 {
                if buckets[i] > 1 {
                    return true;
                }
            }
        }
        count != 0
    }
}

#[test]
fn test_q859() {
    assert_eq!(true, Solution::buddy_strings("ba".to_string(), "ab".to_string()));
    assert_eq!(false, Solution::buddy_strings("ab".to_string(), "ab".to_string()));
    assert_eq!(false, Solution::buddy_strings("abc".to_string(), "abc".to_string()));
    assert_eq!(false, Solution::buddy_strings("abcd".to_string(), "abcd".to_string()));
    assert_eq!(true, Solution::buddy_strings("aba".to_string(), "aba".to_string()));
    assert_eq!(true, Solution::buddy_strings("aa".to_string(), "aa".to_string()));
}