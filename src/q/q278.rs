use crate::q::Solution;

#[allow(unused)]
impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, versions: i32) -> bool {
        // 这个函数其实是leetcode提供的，
        // 就是为了让编译通过
        return true;
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.isBadVersion(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}