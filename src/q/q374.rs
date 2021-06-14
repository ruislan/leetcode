use crate::q::Solution;

// 这个函数实际为leetcode给出，这里为了编译通过简单写了一下
unsafe fn guess(_: i32) -> i32 {
    0
}

#[allow(unused)]
impl Solution {
    unsafe fn guess_number(n: i32) -> i32 {
        // 方法1
        // 二分查找
        // AC 0ms 1.9mb
        let mut lo = 1;
        let mut hi = n;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match guess(mid) {
                0 => return mid,
                1 => lo = mid + 1,
                _ => hi = mid - 1,
            }
        }
        lo
    }
}