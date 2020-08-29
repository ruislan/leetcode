use crate::interview::Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        // 考虑到是字母的排列，所以我们可以认为都是ASCII字符
        // 方法1
        // 构建一个128长度的数组，将s的字符频率统计到数组中
        // 如果数组中的字符频率是0或者是偶数或者只有1个奇数的，那么就能组成回文
        // 否则不是回文
        // Passed 0ms 2.1mb
        // let mut bucket = vec![0; 128];
        // s.bytes().for_each(|ch| bucket[ch as usize] += 1);
        //
        // let mut ones = 0;
        // bucket.into_iter().for_each(|n| ones += n & 1);
        //
        // ones < 2

        // 方法1的改进，直接串烧
        // Passed 0ms 2.1mb
        // s.bytes().fold(vec![0; 128], |mut acc, ch| {
        //     acc[ch as usize] += 1;
        //     acc
        // }).into_iter().fold(0, |ones, n| ones + (n & 1)) < 2

        // 方法2
        // 我们可以使用长度为128的数字作为存储容器
        // 字符串的字节数值即是偏移量
        // 然后我们把偏移后的数字与之前的数字进行异或
        // 最后统计二进制1的个数，如果小于等于1个，则说明是回文，否则不是回文
        // Passed 0ms 2.1mb
        s.bytes().fold(0_u128, |acc, ch| acc ^ (1 << ch)).count_ones() < 2
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_permute_palindrome("".to_string()), true);
    assert_eq!(Solution::can_permute_palindrome("a".to_string()), true);
    assert_eq!(Solution::can_permute_palindrome("aa".to_string()), true);
    assert_eq!(Solution::can_permute_palindrome("ab".to_string()), false);
    assert_eq!(Solution::can_permute_palindrome("abdg".to_string()), false);
    assert_eq!(Solution::can_permute_palindrome("tactcoa".to_string()), true);
    assert_eq!(Solution::can_permute_palindrome("tactcoaodc".to_string()), false);
}