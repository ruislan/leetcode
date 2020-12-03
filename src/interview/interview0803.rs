use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        // 方法1
        // 暴力解决
        // 不知道超时不，多半会超时，因为数据量达到了100万
        // 时间：O(n)，空间：O(1)
        // for i in 0..nums.len() {
        //     if nums[i] == i as i32 { return nums[i]; }
        // }
        // -1

        // 方法2
        // 利用其有序的特性，
        // 那么如果nums[i] > i，
        // 说明nums[i].. nums[i] - i这间都不会相等，我们可以跳过
        // 这样可以让我们快速到达结果，但是如果每个数字都比前面大1，这就是最坏的情况
        // 时间：O(n)，空间：O(1)
        let mut i = 0;
        while i < nums.len() {
            let x = nums[i] as usize;
            if x == i { return x as i32; }
            i += x - i;
        }
        -1
    }
}