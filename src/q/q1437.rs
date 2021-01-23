use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        // 方法1
        // 每次找到==1的，
        // 就看与前面的1的距离是否是在k范围内
        // 如果是就返回false
        // 注意第一次的1是用来初始化的
        let mut last_one = -1;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                if last_one >= 0 && i as i32 - last_one - 1 < k {
                    return false;
                }
                last_one = i as i32;
            }
        }
        true
    }
}