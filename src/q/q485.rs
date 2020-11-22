use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut c = false;
        let mut count = 0;
        for n in nums {
            if n == 1 {
                if c {
                    count += 1;
                } else {
                    c = true;
                    count = 1;
                }
                if count > max {
                    max = count;
                }
            } else {
                c = false;
                count = 0;
            }
        }
        max
    }
}