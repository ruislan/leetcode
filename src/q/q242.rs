use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 方法1
        // 构建一个26长度的数组freq_s和freq_t，
        // 迭代s，将字符的频率放入freq_s
        // 迭代t，将字符的频率放入freq_t
        // 判断两个数组是否相等即可
        // Passed 0ms 2.3mb 时间O(n) 空间O(2n)
        // let (mut freq_s, mut freq_t) = (vec![0; 26], vec![0; 26]);
        // s.bytes().for_each(|ch| freq_s[ch as usize - 97] += 1);
        // t.bytes().for_each(|ch| freq_t[ch as usize - 97] += 1);
        // freq_s == freq_t

        // 方法2
        // 优化方法1，只需要1个数组
        // Passed 0ms 2.1mb 时间O(n) 空间O(n)
        let mut freq = vec![0; 26];
        s.bytes().for_each(|ch| freq[ch as usize - 97] += 1);
        t.bytes().for_each(|ch| freq[ch as usize - 97] -= 1);
        freq.into_iter().filter(|&n| n != 0).count() == 0

        // 方法3
        // 排序s和t，然后比较
        // Passed 0ms 2mb 时间O(nlogn) 空间O(1)
        // let (mut s, mut t) = (s.into_bytes(), t.into_bytes());
        // s.sort_unstable();
        // t.sort_unstable();
        // s == t
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_anagram("".to_string(), "".to_string()), true);
    assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
}