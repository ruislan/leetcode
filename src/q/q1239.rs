use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        // 方法1
        // 1.求出每个字符串的频率
        // 2.回溯列出所有的组合
        // AC 0ms 2.1mb
        fn dfs(masks: &Vec<i32>, i: usize, mask: i32, answer: &mut i32) {
            if i == masks.len() {
                *answer = (*answer).max(mask.count_ones() as i32);
                return;
            }
            if (mask & masks[i]) == 0 {
                dfs(masks, i + 1, mask | masks[i], answer);
            }
            dfs(masks, i + 1, mask, answer);
        }

        let mut masks = Vec::new();
        for s in arr.iter() {
            let mut mask = 0;
            for b in s.bytes() {
                let ch = b - b'a';
                if (mask >> ch) & 1 != 0 {
                    mask = 0;
                    break;
                }
                mask = mask | (1 << ch);
            }
            if mask > 0 {
                masks.push(mask);
            }
        }
        let mut answer = 0;
        dfs(&masks, 0, 0, &mut answer);
        answer
    }
}