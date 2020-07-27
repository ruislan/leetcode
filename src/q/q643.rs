mod q643 {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        for i in 0..k as usize {
            sum += nums[i];
        }
        let mut sum0 = sum;
        for i in k as usize..nums.len() {
            sum0 = sum0 + nums[i] - nums[i - k as usize];
            if sum0 > sum { sum = sum0; }
        }
        sum as f64 / k as f64
    }
}