use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // 方法1
        // 显然s1.len()只能小于或者等于s2.len()
        // 这里要考虑s1和s2中的子串是忽略排列等价的
        // 那么不难想到的第一点就是如果s1排序后和s2的子串排序后一定是相等
        // 但是显然排序很浪费时间，那么发现全是小写组成
        // 那么只需要用数组来得到字符的频率就可以进行比较
        // 频率相同的，自然就相同
        // 所以我们解决了相等的问题
        // 那么接着来解决s2的子串问题，
        // 显然，s2的子串可以看成是一个s1长度的窗口
        // 我们向前滑动的时候，进入窗口一个字符，出窗口一个字符
        // 这个时候只需要在出窗口的时候将字符的频率减去
        // 入窗口的时候将字符的频率加上
        // 然后再进行比较就可以了
        if s1.len() > s2.len() { return false; }

        let mut freq = vec![0; 26];
        for ch in s1.bytes() { freq[ch as usize - 97] += 1; }
        let s2 = s2.into_bytes();

        let mut window_freq = vec![0; 26];
        let mut window = std::collections::VecDeque::new();
        let k = s1.len();
        for i in 0..k {
            window_freq[s2[i] as usize - 97] += 1;
            window.push_back(s2[i]);
        }
        if window_freq == freq { return true; }

        for i in k..s2.len() {
            let remove = window.pop_front().unwrap();
            let add = s2[i];
            window_freq[remove as usize - 97] -= 1;
            window.push_back(add);
            window_freq[add as usize - 97] += 1;
            if window_freq == freq { return true; }
        }
        false
    }
}