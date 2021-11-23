use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        // 方法1
        // 交换一次的情况下要相等
        // 1. 那么首先要保证的就是两个字符串的长度要相等
        // 2. 然后两个字符串的字符频率要一样
        // 3. 接着检查两个字符串的对位不相同的字符数量是否为2或者没有
        //   3.1 如果为2，那一定可以交换
        //   3.2 如果没有，那么就要判断是否至少有一个字符的频率要大于等于2（至少2个才可以交换）
        // AC 0ms 2.2mb 34/34
        let s: Vec<char> = s.chars().collect();
        let goal: Vec<char> = goal.chars().collect();
        if s.len() != goal.len() { return false; }
        let mut diff = 0;
        let mut freq_s = vec![0; 26];
        let mut freq_goal = vec![0; 26];
        for i in 0..s.len() {
            if s[i] != goal[i] { diff += 1; }
            freq_s[(s[i] as u8 - b'a') as usize] += 1;
            freq_goal[(goal[i] as u8 - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if freq_s[i] != freq_goal[i] { return false; }
        }
        if diff == 0 {
            for i in 0..26 {
                if freq_s[i] > 1 { return true; }
            }
        }
        diff == 2
    }
}