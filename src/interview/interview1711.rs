use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        // 方法1
        // 双指针
        // 首先p1找到word1，然后p2找到word2，计算距离
        // 如果p1 小于 p2，则移动p1，反之，移动p2
        // O(n) O(1)
        // AC 40ms 8.1mb
        let n = words.len();
        let mut p1 = 0;
        let mut p2 = 0;
        let mut answer = i32::MAX;
        while p1 < n && p2 < n {
            while p1 < n && words[p1] != word1 { p1 += 1; }
            while p2 < n && words[p2] != word2 { p2 += 1; }
            if p1 < n && p2 < n {
                answer = (p1 as i32 - p2 as i32).abs().min(answer);
            }
            if p1 < p2 { p1 += 1; }
            else { p2 += 1;}
        }
        answer

        // 方法2
        // hashmap优化的双指针解法
        // hashmap存储了所有的字符的出现的位置（从小到大）
        // 然后两个字符的位置，通过双指针来比较，
        // 如果p1的值大于p2，则移动p2，反之移动p1
        // O(n) O(n)
        // AC 72ms 9.7mb
        // let mut map = std::collections::HashMap::new();
        // for (i, word) in words.iter().enumerate() {
        //     let v = map.entry(word).or_insert(Vec::new());
        //     v.push(i as i32);
        // }
        // if let Some(v1) = map.get(&word1) {
        //     if let Some(v2) = map.get(&word2) {
        //         let mut p1 = 0;
        //         let mut p2 = 0;
        //         let n1 = v1.len();
        //         let n2 = v2.len();
        //         let mut answer = i32::MAX;
        //         while p1 < n1 && p2 < n2 {
        //             answer = answer.min((v1[p1] - v2[p2]).abs());
        //             if v1[p1] > v2[p2] {
        //                 p2 += 1;
        //             } else {
        //                 p1 += 1;
        //             }
        //         }
        //         answer
        //     } else {
        //         v1[0]
        //     }
        // } else {
        //     0
        // }
    }
}