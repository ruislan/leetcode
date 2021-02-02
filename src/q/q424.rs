use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // 方法1
        // 字符串出现不同的时候，就开始替换，同时记录下第一个不同的位置
        // 当替换数量达到上限的时候，就从第一个不同的位置重新开始上述步骤
        // 这里要注意的是如果迭代到了最后还有未替换的，那么需要替换前面的字符
        // 以及如果K=0的时候，也就是不需要替换的时候，直接计算连续串就行了
        // AC 424ms 2mb
        // let n = s.len();
        // if n < 2 { return n as i32; }
        // let s = s.into_bytes();
        // let mut answer = 0;
        // let mut current = s[0];
        // let mut i = 0;
        // let mut j = 0;
        // let mut cnt = 0;
        // let mut used = 0;
        // while i < n {
        //     if s[i] == current {
        //         cnt += 1;
        //     } else {
        //         if used < k {
        //             used += 1;
        //             cnt += 1;
        //         } else {
        //             if k > 0 { i = j; }
        //             current = s[i];
        //             used = 0;
        //             cnt = 1;
        //         }
        //         if used == 1 { j = i; }
        //     }
        //     if i == n - 1 { cnt += k - used; }
        //     answer = answer.max(cnt.min(n as i32));
        //     i += 1;
        // }
        // answer

        // 方法2
        // 滑动窗口
        // 我们先把问题先简化一下成K=0的时候的窗口的情况
        // 这个过程是我们从0开始向右探测，如果窗口内的字符都一样
        // 那么窗口就扩大了1，如果不一样，那么窗口无法扩大，就只能右移
        // 例如：AAABBA
        // 第一步： 0开始到1，窗口长度2，窗口内的字符A=2
        // 第二步： 0开始到2，窗口长度3， 窗口内的字符A=3
        // 第三步： 0开始到3，窗口长度4，窗口内的字符A=3，B=1
        //        由于窗口内的字符有不同的，所以滑动左边+1，
        //         滑动之后，从1开始到3， 窗口长度3
        // 第四步： 1开始到4，窗口长度4，窗口内的字符A=2，B=2
        //        由于窗口内的字符有不同的，所以滑动左边+1，
        //         滑动之后，从2开始到4， 窗口长度3
        // 第五步： 2开始到5，窗口长度4，窗口内的字符A=3，B=2
        //        由于窗口内的字符有不同的，所以滑动左边+1，
        //         滑动之后，从3开始到5， 窗口长度3
        // 已经到达最后，窗口长度并没有增加，所以窗口的长度就是最长的
        // 简化后的滑动窗口搞清楚了之后，那么就是可以替换的了
        // 这里我们步骤其实还是相同，只是我们允许了有k个不同的字符而已
        // 理解了这个思路就可以开始编码了
        // 我们用freq[26]来存储窗口内的字符的频率，
        // 再用一个max_freq来存储窗口内频率最大的字符数
        // 这里有个小思考，为什么不去维护freq_max，
        // 例如窗口变化的时候我们迭代freq查找最大，我们依然可以得到正确的答案
        // 其实很简单，首先窗口代表的是当前最大，它不会缩小了，
        // 然后窗口中的freq_max也代表了当前窗口最大的字符频率，
        // 窗口内无论什么字符没有到freq_max的都是要被替换的，
        // 也就是说，如果连freq_max的超过了替换量，更不要说没有到达freq_max，替换的只能更多
        // 所以，除非freq_max增加，窗口才会有扩大的机会
        // AC 0ms 2mb
        let n = s.len();
        let k = k as usize;
        let s = s.into_bytes();
        let mut left = 0;
        let mut right = 0;
        let mut freq_max = 0;
        let mut freq = vec![0; 26];
        while right < n {
            freq[(s[right] - b'A') as usize] += 1;
            freq_max = freq_max.max(freq[(s[right] - b'A') as usize]);

            if right - left + 1 - freq_max > k {
                freq[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
            right += 1;
        }
        (right - left) as i32
    }
}