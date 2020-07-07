use crate::Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // 方法1
        // 构建2n长度的数组res，然后迭代范围0..n
        // 然后再res.push(nums[i])和nums[i + n]
        // 输出res

        // 方法2
        // 通过点n进行分离nums成为nums0,nums1，
        // 然后zip两个数组，
        // 再map成vec![nums0[i],nums1[i]]
        // 再flatten输出
        vec![]
    }
}