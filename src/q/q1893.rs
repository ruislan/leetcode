use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        // 方法1
        // 用hashmap存储每个点位，如果被覆盖，则为true
        // 检查left..=right是否被覆盖
        // AC 0ms 2.2mb
        let mut cover = vec![false; 51];
        let n = ranges.len();
        for i in 0..n {
            for j in ranges[i][0]..=ranges[i][1] {
                cover[j as usize] = true;
            }
        }
        for i in left..=right {
            if !cover[i as usize] { return false; }
        }
        true
    }
}