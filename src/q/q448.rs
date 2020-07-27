mod q448 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut bucket = vec![0; len];
        for num in nums {
            bucket[num as usize - 1] += 1;
        }

        let mut res = Vec::new();
        for i in 0..len {
            if bucket[i] == 0 {
                res.push(i as i32 + 1);
            }
        }

        res
    }
}