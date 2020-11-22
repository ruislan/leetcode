use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        // 方法1
        // let mut a_count = 0;
        // let mut b_count = 0;
        // let mut s_chars: Vec<char> = secret.chars().collect();
        // let mut g_chars: Vec<char> = guess.chars().collect();
        //
        // let mut i = 0;
        // loop {
        //     let mut removed = false;
        //     if i >= s_chars.len() { break; }
        //     if s_chars[i] == g_chars[i] {
        //         a_count += 1;
        //         s_chars.remove(i);
        //         g_chars.remove(i);
        //         removed = true;
        //         if i > 0 {
        //             i -= 1;
        //         }
        //     }
        //     if !removed {
        //         i += 1;
        //     }
        // }
        //
        // let mut i = 0;
        // loop {
        //     if i >= s_chars.len() { break; }
        //
        //     let mut j = 0;
        //     let mut removed = false;
        //     loop {
        //         if j >= g_chars.len() { break; }
        //         if g_chars[j] == s_chars[i] {
        //             b_count += 1;
        //             s_chars.remove(i);
        //             g_chars.remove(j);
        //             removed = true;
        //             if i > 0 {
        //                 i -= 1;
        //             }
        //         }
        //         j += 1;
        //     }
        //     if !removed {
        //         i += 1;
        //     }
        // }
        //
        // let mut res = String::new();
        // res.push_str(&a_count.to_string());
        // res.push('A');
        // res.push_str(&b_count.to_string());
        // res.push('B');
        // res

        // 方法2
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