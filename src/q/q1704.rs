use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        // 方法1
        // 求出半段初始位置half
        // 前半段出现元音增加元音数
        // 后半段出现元音减少元音数
        // 饭后元音数是否为0
        // Passed 0ms 2mb
        let mut half = s.len() >> 1;
        let mut answer = 0;
        for (i, ch) in s.char_indices() {
            if "aeiouAEIOU".find(ch).is_some() {
                if i < half {
                    answer += 1;
                } else {
                    answer -= 1;
                }
            }
        }
        answer == 0
    }
}