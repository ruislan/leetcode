use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法与q451重复，更名为frequency_sort_1636
    pub fn frequency_sort_1636(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 统计数字的频率
        // 根据频率的大小排序，如果频率相等，则根据数字的大小排序
        // 然后再展开即可
        // Passed 0ms 2.2mb
        let mut map = std::collections::HashMap::new();
        for n in nums {
            let freq = map.entry(n).or_insert(0);
            *freq += 1;
        }
        let mut freq: Vec<(i32, i32)> = map.into_iter().map(|x| x).collect();
        freq.sort_unstable_by(|a, b| {
            let ordering = a.1.cmp(&b.1);  // 按频率升序
            if ordering == std::cmp::Ordering::Equal {
                b.0.cmp(&a.0)  // 按数值降序
            } else {
                ordering
            }
        });
        freq.into_iter().map(|x| vec![x.0; x.1 as usize]).flatten().collect()
    }
}