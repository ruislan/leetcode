use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        // 方法1
        // 26个字母存储他们自己最长的按键时间
        // 从他们中找到按件最长的那个
        // 相等的取字母排序后面的就行
        // Passed 0ms 2.1mb
        let mut times = vec![0; 26];
        let keys_pressed: Vec<u8> = keys_pressed.bytes().collect();
        times[keys_pressed[0] as usize - 97] = release_times[0];

        for i in 1..release_times.len() {
            let interval = release_times[i] - release_times[i - 1];
            times[keys_pressed[i] as usize - 97] = times[keys_pressed[i] as usize - 97].max(interval);
        }

        let mut answer = 0;
        let mut max = 0;
        for i in 0..26 {
            if max <= times[i] {
                max = times[i];
                answer = i;
            }
        }
        (answer as u8 + 97) as char
    }
}