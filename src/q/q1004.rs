use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 将连续的1看成是一个滑动的窗口
        // 当我们尝试扩展窗口的时候，如果有0，我们记录0的个数
        // 如果这个个数大于k,说明我们的扩展无法满足条件
        // 所以我们要将窗口左侧收缩1，
        // 以保证当前最大的窗口长度是上一次的满足条件的长度
        // 当收缩的这个值是0时，我们减少0的个数，
        // 以便下一次扩展窗口处于正确的状态下
        // AC 8ms 2mb
        let n = a.len();
        let mut answer = 0;
        let mut lo = 0;
        let mut hi = 0;
        let mut zeros = 0;
        while hi < n {
            if a[hi] != 1 { zeros += 1; }
            hi += 1;
            if zeros > k {
                if a[lo] == 0 { zeros -= 1; }
                lo += 1;
            }
            answer = answer.max(hi - lo)
        }
        answer as i32
    }
}