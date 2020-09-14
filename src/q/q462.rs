use crate::q::Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        // 方法1
        // let mut min_moves = i32::max_value();
        // for n in nums.iter() {
        //     let mut moves = 0;
        //     for t in nums.iter() {
        //         moves += (*t - *n).abs();
        //         if moves < 0 { break; }
        //     }
        //     if moves >= 0 && moves < min_moves { min_moves = moves; }
        // }
        // min_moves
        
        // 方法2
        nums.sort();
        let mut min_moves = 0;
        for n in nums.iter() {
            min_moves += (nums[nums.len() / 2] - n).abs();
        }
        min_moves
    }
}