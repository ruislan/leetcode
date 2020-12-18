use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        // 方法1
        // 将数组中的每个字符串分成奇数排序+偶数排序
        // 然后将相同的分组，统计分组数量即可
        // Passed 0ms 2.1mb
        let mut map = std::collections::HashMap::new();
        for s in a {
            let mut odds = Vec::new();
            let mut evens = Vec::new();
            for (i, x) in s.bytes().enumerate() {
                if i & 1 == 1 { odds.push(x); } else { evens.push(x); }
            }
            odds.sort_unstable();
            evens.sort_unstable();
            odds.append(&mut evens);
            *map.entry(odds).or_insert(0) += 1;
        }
        map.len() as i32
    }
}