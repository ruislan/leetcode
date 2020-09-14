use crate::q::Solution;

impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut ng = Vec::new();
        for i in 0..nums.len() {
            ng.push((nums[i], i));
        }
        ng.sort_by(|a, b| b.0.cmp(&a.0));
        let mut res = vec![String::new(); ng.len()];
        for i in 0..ng.len() {
            if i > 2 {
                res[ng[i].1] = (i + 1).to_string();
            } else if i == 2 {
                res[ng[i].1] = String::from("Bronze Medal");
            } else if i == 1 {
                res[ng[i].1] = String::from("Silver Medal");
            } else {
                res[ng[i].1] = String::from("Gold Medal");
            }
        }
        res
    }
}