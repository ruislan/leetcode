use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        // 方法1
        // 我们维护一个滑动窗口，假设我们这个窗口已经在中间位置，
        // 我们迭代尝试扩展窗口，也即是right += 1
        // 当窗口中的window_cost比max_cost要大时，我们收缩窗口的左边，也即是left -= 1
        // 由于我们减少了窗口的左边，所以window_cost也相应的减少
        // 每次尝试扩展窗口的时候，我们计算窗口的大小，保留下最大的那个就是结果
        // AC 0ms 2.3mb
        let mut left = 0;
        let mut right = 0;
        let mut window_cost = 0;
        let n = s.len();
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut answer = 0;
        while right < n {
            window_cost += (s[right] as i32 - t[right] as i32).abs();
            if window_cost > max_cost {
                window_cost -= (s[left] as i32 - t[left] as i32).abs();
                left += 1;
            }
            right += 1;
            answer = answer.max(right - left);
        }
        answer as i32
    }
}