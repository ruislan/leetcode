use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        // 方法1
        // n = word.len()
        // 首先找到word第一个匹配的位置i，
        // 然后比对i + len.. i + 2 * len - 1的位置是否相等
        // 相等了之后继续向前移动，直到结束位置 > sequence.len()
        // 返回记数结果
        0
    }
}