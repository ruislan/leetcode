use crate::q::Solution;

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut res = Vec::new();
        let mut num = 0;
        for n in a {
            num = ((num << 1) + n) % 5;
            res.push(num == 0);
        }
        res
    }
}