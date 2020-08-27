use crate::interview::Solution;

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        // 方法1
        // 所有字符是否都不相同，我们构建一个hashset，如果存在重复字符，则返回false
        // 否则返回true
        // Passed 0ms 2.1mb
        // let mut sets = std::collections::HashSet::new();
        // astr.bytes().find(|&x| !sets.insert(x)).is_none()

        // 方法2
        // 将所有的字节排序，然后迭代字节数组，如果有两个相同，则返回false
        // 快速排序是O(nlogn)
        // Passed 0ms 2.1mb
        // let mut vec = astr.bytes().collect::<Vec<u8>>();
        // vec.sort_unstable();
        // (1..vec.len()).find(|&x| vec[x - 1] == vec[x]).is_none()

        // 方法3
        // 一个字节是8位，总共有256种字符，所以我们不要hashset的话，需要256长度的数组
        // Passed 0ms 2.1mb
        // let mut arr = vec![0; 256];
        // astr.bytes().find(|&x| {
        //     let x = x as usize;
        //     if arr[x] == 1 { return true; }
        //     arr[x] = 1;
        //     return false;
        // }).is_none()

        // 方法4
        // 如果我们要节约空间，也即是内存要用O(1)级别
        // 考虑到只需要找出两个相同即可以返回false，那么我们完全可以使用二进制数组
        // 也即是说如果有一个长度是256的数字0，那么它完全可以作为容器，rust没有256的数字
        // 它只有128的，所以我们用两个128的数字来表示也可以。
        // 每一个数字的字节值就是需要偏移的位置，如果那个位置有1，则返回false，如果没有则为1
        // 注意是位运算
        let (mut b0, mut b1) = (0_u128, 0_u128);
        astr.bytes().find(|&x| {
            if x < 128 {
                let t = b0;
                b0 |= 1 << x;
                t == b0
            } else {
                let t = b1;
                b1 |= 1 << (x - 128);
                t == b1
            }
        }).is_none()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_unique("".to_string()), true);
    assert_eq!(Solution::is_unique("lAAtcode".to_string()), false);
    assert_eq!(Solution::is_unique("leetcode".to_string()), false);
    assert_eq!(Solution::is_unique("abc".to_string()), true);
    assert_eq!(Solution::is_unique("AabB".to_string()), true);
    assert_eq!(Solution::is_unique("  ".to_string()), false);
    assert_eq!(Solution::is_unique(" ".to_string()), true);
    assert_eq!(Solution::is_unique(" ".to_string()), true);
    assert_eq!(Solution::is_unique("中国".to_string()), true);
    assert_eq!(Solution::is_unique("中中国".to_string()), false);
}