use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        // 方法1
        // 由于数据量较小，可以暴力解决
        // 将s转化成bytes数组
        // i从0..chars.len() - 1，j从chars.len() -1 down to i + 1
        // 如果找到了相等的字符，那么自然就是最大值
        // 找到每个相等字符的最大值，选最大的那个返回
        // 时间O(n^2) 空间O(1)
        // let mut max = -1;
        // let mut chars = s.into_bytes();
        // for i in 0..chars.len() - 1 {
        //     let mut j = chars.len() - 1;
        //     while j > i {
        //         if chars[i] == chars[j] { break; }
        //         j -= 1;
        //     }
        //     max = max.max((j - i) as i32 - 1);
        // }
        // max

        // 方法2
        // 由于只含有小写字母，所以可以创建freq数组记录他第一次和最后一次出现的位置
        // freq[i] = 位置(min， max)
        // 一次迭代就能把所有的位置记录下来
        // 迭代freq，求出freq[i] = max - min - 1，找出max最大的那个
        let mut max = -1;
        let mut freq = vec![(s.len(), 0); 26];
        for (i, ch) in s.into_bytes().into_iter().enumerate() {
            freq[ch as usize - 97].0 = freq[ch as usize - 97].0.min(i);
            freq[ch as usize - 97].1 = freq[ch as usize - 97].1.max(i);
        }
        for dis in freq {
            max = max.max((dis.1 - dis.0) as i32 - 1);
        }
        max
    }
}