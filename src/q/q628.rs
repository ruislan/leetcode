use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        // 方法1
        // 比较后三个和前二后1的乘积的大小即可
        // Passed 8ms
        // let mut nums = nums;
        // nums.sort();
        // std::cmp::max(nums[0] * nums[1] * nums[nums.len() - 1], nums[nums.len() - 3] * nums[nums.len() - 2] * nums[nums.len() - 1])

        // 方法2
        // 方法1用了排序O(nlogn)，方法2直接遍历找出最小两个和最大三个共5个数字O(n)
        // Passed 4ms
        let (mut min0, mut min1) = (i32::max_value(), i32::max_value());
        let (mut max0, mut max1, mut max2) = (i32::min_value(), i32::min_value(), i32::min_value());
        for num in nums {
            if num < min0 {
                min1 = min0;
                min0 = num;
            } else if num < min1 {
                min1 = num;
            }

            if num > max0 {
                max2 = max1;
                max1 = max0;
                max0 = num;
            } else if num > max1 {
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max2 = num;
            }
        }
        let sum0 = min0 * min1 * max0;
        let sum1 = max0 * max1 * max2;
        if sum0 > sum1 { sum0 } else { sum1 }
    }
}