use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 方法1
        // 和q274基本相同，这里用二分方法来处理
        // AC 0ms 2.7mb
        let n = citations.len();
        let mut lo = 0;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if citations[mid] >= (n - mid) as i32 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        (n - lo) as i32
    }
}