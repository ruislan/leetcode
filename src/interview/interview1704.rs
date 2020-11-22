use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // 排序，迭代i in 0..nums.len()
        // 第一个i != nums[i]就是，返回
        // 时间O(nlogn) 空间O(1)
        // Passed 4ms 2.1mb
        // let mut nums = nums;
        // nums.sort_unstable();
        // let mut i = 0;
        // while i < nums.len() {
        //     if i as i32 != nums[i] { break; }
        //     i += 1;
        // }
        // i as i32

        // 方法2
        // 创建一个nums.len() + 1长度的数组arr，将nums的数组取出，按照其序号将arr[nums[i]]设置为true
        // 迭代arr，如果arr[i] = false的，返回
        // 时间O(n)，空间O(n)
        // Passed 0ms 2.1mb
        // let mut arr = vec![false; nums.len() + 1];
        // nums.into_iter().for_each(|n| arr[n as usize] = true);
        // arr.into_iter().enumerate().find(|&x| !x.1).unwrap().0 as i32

        // 方法3
        // 求出nums的总和 - 0..=nums.len()的总和，剩下的那个数字就是要返回的数字
        // 时间O(n)，空间O(1)
        // Passed 0ms 2.1mb
        // 朴素写法：
        // (0..=nums.len()).sum::<usize>() as i32 - nums.into_iter().sum::<i32>()
        // 通项公式写法：
        // ((nums.len() + 1) * nums.len() >> 1) as i32 - nums.into_iter().sum::<i32>()

        // 方法4
        // 让nums[i]与i进行异或（nums.len()也需要异或)，
        // 这样相同的数字都会被消掉
        // 留下的数字就是那个要找的
        // 时间O(n)， 空间O(1)
        // Passed 0ms 2mb
        nums.len() as i32 ^ nums.into_iter().enumerate().fold(0, |acc, (i, x)| acc ^ i as i32 ^ x)
    }
}