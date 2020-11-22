use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // if ops.len() < 1 { return m * n; }
        // let mut min_x = 40001;
        // let mut min_y = 40001;
        // for i in 0..ops.len() {
        //     if ops[i][0] < min_x { min_x = ops[i][0]; }
        //     if ops[i][1] < min_y { min_y = ops[i][1]; }
        // }
        // min_x * min_y
        // 方法2
        let mut min_x = m;
        let mut min_y = n;
        for i in 0..ops.len() {
            min_x = std::cmp::min(min_x, ops[i][0]);
            min_y = std::cmp::min(min_y, ops[i][1]);
        }
        min_x * min_y
    }
}