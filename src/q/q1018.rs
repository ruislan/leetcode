use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        // 方法1
        // 根据题目描述我们要得到下一位则是(x << 1) + a[i]
        // 因为每一次我们之前的数字都要变大2倍
        // 但是由于a的长度会超出整形的存储极限
        // 所以我们需要将数字全部压缩到一个范围内，
        // 这里由于是能够被5所整除，那么就只有0，5结尾的数字可以
        // 也就说说，每次我们都让数字在5以内（5就是0，0就是0）
        // 也就是我们的数字x每次都要x %= 5
        // Passed 268ms 38.6mb
        let mut num = 0;
        let mut answer = Vec::new();
        for b in a {
            num = ((num << 1) + b) % 5;
            answer.push(num == 0);
        }
        answer
    }
}