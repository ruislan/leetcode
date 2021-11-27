use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        // 方法1
        // 滑动窗口
        // 窗口的大小就是P的长度
        // 窗口每移动一次，就把窗口的左边的字母移除，把窗口的右边的字母添加
        // 然后比较窗口的字母数量是否符合P的字母数量
        // 如果符合，那么就把窗口的左边的索引加入结果
        // AC 0ms 2.3mb 61/61
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut window = vec![0; 26];
        let mut freq_p = vec![0; 26];
        let mut ans = vec![];

        for i in 0..p.len() { freq_p[p[i] as usize - 97] += 1; }
        for i in 0..s.len() {
            window[s[i] as usize - 97] += 1;
            if i >= p.len() {
                window[s[i - p.len()] as usize - 97] -= 1;
            }
            if window == freq_p {
                ans.push(i as i32 - p.len() as i32 + 1);
            }
        }
        ans
    }
}