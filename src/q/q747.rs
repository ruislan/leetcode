mod q747 {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_i = 0;
        for i in 0..nums.len() {
            if nums[i] > max {
                max = nums[i];
                max_i = i;
            }
        }
        for i in 0..nums.len() {
            if i != max_i && nums[i] * 2 > max { return -1; }
        }
        max_i as i32
    }
}