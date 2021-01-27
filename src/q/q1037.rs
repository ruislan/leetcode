use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        // 方法1
        // 检查3点是否在一条直线就行了
        // 是否是一条直线用斜率公式，注意把除法转移成乘法即可
        (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]) != (points[1][0] - points[0][0]) * (points[2][1] - points[0][1])
    }
}