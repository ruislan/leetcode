use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 方法1
        // AC 64ms 2.1mb
        // if s.len() <= 1 { return s.len() as i32; }
        // let mut max = 0;
        // let chars = s.chars().collect::<Vec<char>>();
        // let len = chars.len();
        // for mut i in 0..len {
        //     let mut temp = String::new();
        //     temp.push(*chars.get(i).unwrap());
        //     for k in (i + 1)..len {
        //         let ch = chars.get(k).unwrap();
        //         if temp.contains(*ch) {
        //             if k - i > max { max = k - i; }
        //             i = k;
        //             break;
        //         } else if k + 1 == len && len - i > max {
        //             max = len - i;
        //             break;
        //         }
        //         temp.push(*ch);
        //     }
        // }
        // max as i32

        // 方法2
        // AC 4ms 2.2mb
        // let chars = s.chars().collect::<Vec<char>>();
        // let mut map_char = std::collections::HashMap::new();
        // let (mut i, mut j, mut ans) = (0, 0, 0);
        // while j < s.len() {
        //     if map_char.contains_key(&chars[j]) {
        //         i = std::cmp::max(*map_char.get(&chars[j]).unwrap(), i);
        //     }
        //     ans = std::cmp::max(ans, j - i + 1);
        //     map_char.insert(chars[j], j + 1);
        //     j += 1;
        // }
        // ans as i32

        // 方法3
        // 这是时隔1年多来重新做这道题哈，记得当时是做了好几个小时才用方法1做出来，
        // 方法2还是看过题解后做的，现在也就不到十分钟吧，真的是成长了不少
        // 看到题就知道要用滑动窗口了，这种计算连续子数组的问题，多半都有滑动窗口的影子
        // 当我们没有出现重复字符串的时候就不停的扩展窗口
        // 当出现了重复字符串，就收缩窗口到刚刚出现的那个重复字符串的位置+1
        // 这样窗口内的字符串又都不重复了
        // 然后取窗口最大的时候就是结果
        // AC 0ms 2mb
        let n = s.len();
        let s = s.into_bytes();
        let mut freq = vec![0; 256];
        let mut answer = 0;
        let mut lo = 0;
        for hi in 0..n {
            freq[s[hi] as usize] += 1;
            if freq[s[hi] as usize] > 1 {
                while s[lo] != s[hi] {
                    freq[s[lo] as usize] -= 1;
                    lo += 1;
                }
                freq[s[lo] as usize] -= 1;
                lo += 1;
            }
            answer = answer.max(hi - lo + 1);
        }
        answer as i32
    }
}