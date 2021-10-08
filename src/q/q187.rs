use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        // 方法1
        // 固定得窗口是10，那么移动窗口，每次进来一个字母，又弹出第一个字符，保持10个字符
        // 每次的10个字符都存储到哈希表中，如果出现重复了，则说明存在重复的序列
        // AC 8ms 5.3mb 31/31
        let mut set = std::collections::HashSet::new();
        let mut ans = std::collections::HashSet::new();
        s.as_bytes().windows(10).for_each(|arr|
            if !set.insert(arr) {
                ans.insert(arr);
            }
        );
        ans.iter().map(|arr| arr.iter().map(|&x| x as char).collect()).collect()
    }
}
