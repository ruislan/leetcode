use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 方法1
        // if nums.len() < 2 { return nums.len() as i32; }
        // let mut num = nums[0];
        // let mut p = 0;
        // for i in 1..nums.len() {
        //     let n = nums[i];
        //     if n > num {
        //         if i - p == 1 {
        //             p += 1;
        //             num = n;
        //         } else if i - p > 1 {
        //             p += 1;
        //             nums[p] = n;
        //             num = n;
        //         }
        //     }
        // }
        // (p + 1) as i32

        // 方法2
        if nums.len() < 2 { return nums.len() as i32; }
        let mut p = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[p] {
                p += 1;
                nums[p] = nums[i];
            }
        }
        (p + 1) as i32
    }
}