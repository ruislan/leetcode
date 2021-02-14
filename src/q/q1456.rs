use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        // 方法1
        // 这是一个固定了窗口长度为k的滑动窗口
        // 每次滑动注意进一个然后出一个
        // 进出看是不是元音，进就加1个，出就减1个
        // AC 0ms 2.1mb
        let mut vowels = vec![false; 26];
        vowels[b'a' as usize - 97] = true;
        vowels[b'e' as usize - 97] = true;
        vowels[b'i' as usize - 97] = true;
        vowels[b'o' as usize - 97] = true;
        vowels[b'u' as usize - 97] = true;

        let s = s.into_bytes();
        let n = s.len();
        let k = k as usize;
        let mut widown_vowels = 0;
        for i in 0..k {
            if vowels[s[i] as usize - 97] {
                widown_vowels += 1;
            }
        }
        let mut answer = widown_vowels;
        for i in k..n {
            if vowels[s[i] as usize - 97] {
                widown_vowels += 1;
            }
            if vowels[s[i - k] as usize - 97] {
                widown_vowels -= 1;
            }
            answer = answer.max(widown_vowels);
        }
        answer
    }
}