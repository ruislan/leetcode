struct NumArray {
    nums: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut cache = vec![0; nums.len() + 1];
        let mut sum = 0i32;
        for i in 0..nums.len() {
            sum += nums[i];
            cache[i + 1] = sum;
        }
        NumArray {
            nums: cache
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.nums[j as usize + 1] - self.nums[i as usize]
    }
}