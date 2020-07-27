mod q665 {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut count = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                if i == 0 || nums[i + 1] >= nums[i - 1] {
                    nums[i] = nums[i + 1];
                } else {
                    nums[i + 1] = nums[i];
                }
                count += 1;
                if count > 1 { return false; }
            }
        }
        true
    }
}