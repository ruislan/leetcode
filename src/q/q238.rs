use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 题目要求不能使用除法，也就是意味着我们不能乘以所有的数字result
        // 然后将result / nums[i]的方式来处理，而且还有一个数组中的元素有可能是0
        // 那么我们尝试求出从左到右和从右到左的前缀乘积和，例如：
        // 原数组  nums:  [1,2,3,4]
        // 左到右  pre:   [1,2,6,24]
        // 右到左  suf:  [24,24,12,4]
        // 假设我们求nums[2] = 3这个位置的乘积，应该得到8
        // 观察pre和suf，发现pre[1] * suf[3]正好等于8
        // 也就是说nums[i] = pre[i - 1] * suf[i + 1]
        // 那么现在就剩下两边了，也就是nums[0]和nums[3]的时候
        // 可以通过将pre的数组前多添加一位的方式得到 [1,1,2,6,24]
        // 以及通过将suf的数组后多添加一位的方式得到 [24,24,12,4,1]
        // 这样一来，pre自然就少一位，suf就可以多取一位。
        // 所以调整等式得到，nums[i] = pre[i] * suf[i + 1]
        // 带入nums[0] = pre[0] * suf[1] = 1 * 24 = 24 = 24/1
        //    nums[1] = pre[1] * suf[2] = 1 * 12 = 12 = 24/2
        //    nums[2] = pre[2] * suf[3] = 2 * 4 = 8 = 24/3
        //    nums[3] = pre[3] * suf[4] = 6 * 1 = 6 = 24/4
        // 与期望一致，且只使用了乘法，和O(n)的时间，O(n)的空间
        // Passed 8ms 3.3mb
        if nums.is_empty() { return Vec::new(); }

        let n = nums.len();
        let mut forward = vec![1; n + 1];
        let mut backward = vec![1; n + 1];
        (0..n).for_each(|i| forward[i + 1] = forward[i] * nums[i]);
        (0..n).rev().for_each(|i| backward[i] = backward[i + 1] * nums[i]);
        (0..n).map(|i| forward[i] * backward[i + 1]).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}