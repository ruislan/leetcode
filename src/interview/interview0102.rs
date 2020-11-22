use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        // 方法1
        // 将两个字符串排序，然后比较
        // Passed 0ms 2.1mb
        // let mut s1 = s1.bytes().collect::<Vec<u8>>();
        // let mut s2 = s2.bytes().collect::<Vec<u8>>();
        // s1.sort_unstable();
        // s2.sort_unstable();
        // s1 == s2

        // 方法2
        // 两个256长度的数组存储字节，比较
        // Passed 0ms 2.1mb
        // let (mut arr1, mut arr2) = (vec![0; 256], vec![0; 256]);
        // s1.bytes().for_each(|x| arr1[x as usize] += 1);
        // s2.bytes().for_each(|x| arr2[x as usize] += 1);
        // arr1 == arr2

        // 方法3
        // 方法2的改进，可以使用一个数组，第二次迭代减去数字即可
        // Passed 0ms 2.1mb
        let mut arr = vec![0; 256];
        s1.bytes().for_each(|x| arr[x as usize] += 1);
        s2.bytes().find(|&x| {
            arr[x as usize] -= 1;
            arr[x as usize] < 0
        }).is_none()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_permutation("".to_string(), "".to_string()), true);
    assert_eq!(Solution::check_permutation("abc".to_string(), "bca".to_string()), true);
    assert_eq!(Solution::check_permutation("abc".to_string(), "bad".to_string()), false);
}