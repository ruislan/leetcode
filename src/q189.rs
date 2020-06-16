mod q_189 {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        reverse(nums, 0, len - 1);
        reverse(nums, 0, k);
        reverse(nums, k, len - k);
    }

    fn reverse(nums: &mut Vec<i32>, from: usize, to: usize) {
        while from <= to {
            let temp = nums[from];
            nums[from] = nums[to];
            nums[to] = temp;
        }
    }
}