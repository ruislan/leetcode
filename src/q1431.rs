struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // 方法1
        // 找出candies中最大的那个数max，
        // 然后迭代candies，如果candies[i] + extra_candies >= max，则map为true，再collect返回
    }
}