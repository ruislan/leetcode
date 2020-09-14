use crate::q::Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted_heights = heights.clone();
        sorted_heights.sort();
        let mut count = 0;
        for i in 0..heights.len() {
            if sorted_heights[i] != heights[i] { count += 1; }
        }
        count
    }
}