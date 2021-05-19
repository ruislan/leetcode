use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        // 方法1
        // 直接hashmap统计词频，然后按照词频排序再按照字母排序即可
        // AC 4ms 2.1mb
        use std::collections::HashMap;
        let mut fq = HashMap::new();
        for word in words {
            *fq.entry(word).or_insert(0) += 1;
        }
        let mut v: Vec<(String, i32)> = fq.into_iter().map(|(k, v)| (k, v)).collect();
        v.sort_unstable_by(|a, b| {
            if b.1 == a.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        let mut answer = Vec::new();
        for i in 0..v.len() {
            answer.push(v[i].0.clone());
            if i as i32 == k - 1 { break; }
        }
        answer
    }
}