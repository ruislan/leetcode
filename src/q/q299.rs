use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        // 方法1
        // 通过计数来记录0-9的频率，如果两个数组中的同一位置的数字相同，则不需要记录，但是bulls增加1
        // 然后直接计算比对两个频率，取相同位置最小的那个就是cows的数量，最后再求和就是总的cows的数量
        // AC 0ms 2.1mb 152/152
        let mut bulls = 0;
        let s_chars: Vec<char> = secret.chars().collect();
        let g_chars: Vec<char> = guess.chars().collect();
        let mut v_s = vec![0u32; 10];
        let mut v_g = vec![0u32; 10];

        for (g, s) in g_chars.iter().zip(s_chars.iter()) {
            if g == s {
                bulls += 1;
            } else {
                v_s[*s as usize - '0' as usize] += 1;
                v_g[*g as usize - '0' as usize] += 1;
            }
        }

        let cows = v_s.iter().zip(v_g.iter()).map(|(s, g)| std::cmp::min(s, g)).sum::<u32>();

        format!("{}A{}B", bulls, cows)
    }
}