use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // 方法1
        // 维护一个滑动窗口
        // 这个窗口不停的向右扩展，
        //    如果当前字符串是t包含的，那么我们加入到window_freq中，
        //    当窗口内（window_freq)已经包含了t字符串中所有的字符
        //    且字符最少的频率也大于或者等于t中的那个字符频率
        //    说明窗口已经满足需求，这个时候窗口的大小就是一个可能的解
        //    然后，删除掉窗口的左端的那个字符，如果这个字符是t包含的，
        //    那么window_freq就减去频率（由于每次都会尝试左端收缩，所以
        //    这里删除的那个字符一定是t包含的）
        //    接着，尝试将窗口左端不断收缩到刚好是t包含的字符
        // 循环结束。
        // 如果没有找到最小的解，那么min的长度会等于我们的初始值s.len() + 1，返回空串。
        // 否则，返回最小的那个窗口区间的字符串即可。

        let mut k = t.len();
        let n = s.len();
        let s = s.into_bytes();
        let t = t.into_bytes();

        let mut t_freq = vec![0; 123];
        for i in 0..k { t_freq[t[i] as usize] += 1; }

        let mut window_freq = vec![0; 123];
        let mut lo = 0;
        let mut hi = 0;
        let mut min = n + 1;
        let mut window_min = (lo, hi + 1);

        let check = |window_freq: &Vec<i32>, t_freq: &Vec<i32>| -> bool {
            for i in 0..123 {
                if window_freq[i as usize] < t_freq[i as usize] {
                    return false;
                }
            }
            true
        };

        while hi < n {
            if t_freq[s[hi] as usize] > 0 {
                window_freq[s[hi] as usize] += 1;
            }
            hi += 1;
            if check(&window_freq, &t_freq) {
                if min > hi - lo {
                    window_min = (lo, hi);
                    min = hi - lo;
                }
                if window_freq[s[lo] as usize] > 0 {
                    window_freq[s[lo] as usize] -= 1;
                }
                lo += 1;
            }
            while lo < hi && t_freq[s[lo] as usize] == 0 {
                lo += 1;
            }
        }

        if min == n + 1 {
            String::new()
        } else {
            (window_min.0..window_min.1).map(|i| s[i] as char).collect()
        }
    }
}