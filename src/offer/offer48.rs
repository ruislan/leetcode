use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 方法1
        // 利用滑动窗口很好解决
        // 当我们没有出现重复字符串的时候就不停的扩展窗口
        // 当出现了重复字符串，就收缩窗口到刚刚出现的那个重复字符串的位置+1
        // 这样窗口内的字符串又都不重复了
        // 然后取窗口最大的时候就是结果
        // AC 4ms 2.1mb
        let n = s.len();
        let s = s.into_bytes();
        let mut freq = vec![0; 256];
        let mut answer = 0;
        let mut lo = 0;
        for hi in 0..n {
            freq[s[hi] as usize] += 1;
            if freq[s[hi] as usize] > 1 {
                while s[lo] != s[hi] {
                    freq[s[lo] as usize] -= 1;
                    lo += 1;
                }
                freq[s[lo] as usize] -= 1;
                lo += 1;
            }
            answer = answer.max(hi - lo + 1);
        }
        answer as i32
    }
}