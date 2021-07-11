use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 方法1
        // 排序之后，我们从大到小的检测是不是满足条件即可
        // 唯一要注意的是rust的usize的溢出就大于0了，
        // 这个时候会出错误，所以我们要增加一个小N的条件
        // O(nlogn)
        // AC 0ms 2.2mb
        let mut citations = citations;
        citations.sort_unstable();
        let n = citations.len();
        let mut h = 0;
        let mut i = n - 1;
        while i >= 0 && i < n && citations[i] > h {
            h += 1;
            i -= 1;
        }
        h
    }
}