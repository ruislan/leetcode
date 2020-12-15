use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        // 方法1
        // 将allowed放入set，然后迭代words，如果出现没有在allowed中的，那么就是错误的
        // passed 20ms 2.6mb
        // let allowed: std::collections::HashSet<u8> = allowed.into_bytes().into_iter().collect();
        //
        // let mut answer = 0;
        // for word in words {
        //     let mut valid = true;
        //     for ch in word.bytes() {
        //         if !allowed.contains(&ch) {
        //             valid = false;
        //             break;
        //         }
        //     }
        //     if valid { answer += 1; }
        // }
        // answer

        // 方法2
        // 用桶优化一下
        // Passed 12ms 2.6mb
        let mut sets = vec![0; 26];
        for x in allowed.into_bytes() {
            sets[(x - 97) as usize] += 1;
        }

        let mut answer = 0;
        for word in words {
            let mut valid = true;
            for x in word.into_bytes() {
                if sets[(x - 97) as usize] == 0 {
                    valid = false;
                    break;
                }
            }
            if valid { answer += 1; }
        }
        answer
    }
}