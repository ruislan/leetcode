mod q283 {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // 方法1
        // for i in 0..nums.len() {
        //     for j in (i + 1)..nums.len() {
        //         if nums[i] == 0 {
        //             nums.swap(i, j);
        //         }
        //     }
        // }
        
        // 方法2
        let mut last = 0;
        for curr in 0..nums.len() {
            if nums[curr] != 0 {
                nums.swap(curr, last);
                last += 1;
            }
        }
    }
}