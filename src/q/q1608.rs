use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        // 方法1
        // 暴力解法
        // 直接按照题目描述，迭代x in 0..nums.len()
        // 然后比较x与nums[i]，统计nums[i] >= x的数量是否和x相等
        // 相等返回结果
        // 不相等，返回-1
        (0..=nums.len() as i32).find(|&x| x == (nums.iter().filter(|&&num| num >= x).count() as i32)).unwrap_or(-1)
    }
}