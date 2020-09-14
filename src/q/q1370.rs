use crate::q::Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        // 方法1
        // 将所有的字符的频率放入0-25的数组bag中，
        // 按照规则，先从左到右选择一次，再从右到左选择一次，再从左到右，直到没有字符为止（来回扫swipe）
        let mut bag = vec![0; 26];
        s.bytes().for_each(|ch| bag[(ch - b'a') as usize] += 1);
        let (mut res, mut amount) = (String::new(), s.len());
        while amount > 0 {
            for i in 0..bag.len() {
                if bag[i] > 0 {
                    res.push((i as u8 + b'a') as char);
                    bag[i] -= 1;
                    amount -= 1;
                }
            }
            for i in (0..bag.len()).rev() {
                if bag[i] > 0 {
                    res.push((i as u8 + b'a') as char);
                    bag[i] -= 1;
                    amount -= 1;
                }
            }
        }
        res
    }
}

#[test]
fn test_q1370() {
    // assert_eq!(Solution::sort_string("".to_string()), "".to_string());
    assert_eq!(Solution::sort_string("aaaabbbbcccc".to_string()), "abccbaabccba".to_string());
    assert_eq!(Solution::sort_string("rat".to_string()), "art".to_string());
    assert_eq!(Solution::sort_string("leetcode".to_string()), "cdelotee".to_string());
    assert_eq!(Solution::sort_string("ggggggg".to_string()), "ggggggg".to_string());
    assert_eq!(Solution::sort_string("spo".to_string()), "ops".to_string());
    assert_eq!(Solution::sort_string("g".to_string()), "g".to_string());
}