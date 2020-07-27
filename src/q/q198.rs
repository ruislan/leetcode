mod q198 {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev_max = 0;
        let mut curr_max = 0;
        for x in nums {
            let temp = curr_max;
            curr_max = std::cmp::max(prev_max + x, curr_max);
            prev_max = temp;
        }
        curr_max
    }
}