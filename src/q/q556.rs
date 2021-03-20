use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        // 方法1
        // 这个算法还挺有名的哈，就叫next permutation
        // 简单来说
        // 先是从尾向头找到第一个递减的数字，这个数字我们是要改变的，记为n1，索引为i
        // 然后从尾向n1找到第一个大于n1的数字，这个数字就是要去和n1交换的，记为n2，索引为j
        // 接着交换n1和n2
        // 最后从原n1的位置的后一个位置开始到尾整个反转即可
        // 例如: 1254321
        // 首先找到第一个n1是2，索引是1
        // 然后从54321中找到刚好大于2的是3，索引是4
        // 接着交换n1和n2，得到1354221
        // 最后把54221反转得到12245，整个结果就是1312245
        // AC 0ms 2mb
        let mut bytes: Vec<u8> = n.to_string().bytes().collect();
        let mut i = bytes.len() - 1;
        while i > 0 && bytes[i] <= bytes[i - 1] { i -= 1; }
        if i == 0 { return -1; }
        let mut j = bytes.len() - 1;
        while bytes[j] <= bytes[i - 1] { j -= 1; }
        bytes.swap(i - 1, j);
        bytes[i..].reverse();
        let mut answer = 0_i64;
        for i in 0..bytes.len() {
            answer = answer * 10 + (bytes[i] as i64 - b'0' as i64);
        }
        if answer > i32::MAX as i64 { -1 } else { answer as i32 }
    }
}