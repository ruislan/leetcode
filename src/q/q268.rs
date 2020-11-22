use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // nums.sort();
        // let mut lost = 0i32;
        // for i in 0..nums.len() {
        //     if nums[i] - lost != 0 {
        //         break;
        //     }
        //     lost += 1;
        // }
        // lost

        // 方法2
        // if nums.len() == 1 && nums[0] == 0 { return 1; }
        // let mut sum = 0i32;
        // let mut max = 0i32;
        // let mut min = i32::max_value();
        // for i in 0..nums.len() {
        //     if nums[i] > max { max = nums[i]; }
        //     if nums[i] < min { min = nums[i]; }
        //     sum += nums[i];
        // }
        // let lost = (0..=max).sum::<i32>() - sum;
        // if min != 0 { return 0; }
        // if max > 0 && lost == 0 { return max + 1; }
        // return lost;

        // 方法3
        let mut missing = nums.len() as i32;
        for i in 0..nums.len() {
            missing ^= nums[i] ^ i as i32;
        }
        missing
    }
}