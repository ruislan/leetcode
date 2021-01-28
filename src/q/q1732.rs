use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        // 方法1
        // gain存储的是逐步的高度差
        // 那么就依次加上g就是当前这个点的海拔高度
        // 然后找出最大的那个就行了
        // AC 0ms 2mb
        let mut altitude = 0;
        let mut answer = 0;
        for g in gain {
            altitude += g;
            answer = answer.max(altitude);
        }
        answer
    }
}