use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // 方法1
        // 暴力法
        // Passed 36ms 2.1mb
        // for i in 0..nums.len() {
        //     for j in (i + 1)..nums.len() {
        //         if nums[i] == 0 {
        //             nums.swap(i, j);
        //         }
        //     }
        // }

        // 方法2
        // 将正确的数字放入正确的位置
        // 我们用一个ptr来表示当前位置应该放入的正确数字，从0开始
        // 当我们遇到第一个不为0的数字，我们就将其与ptr位置的数字进行交换，
        // ptr向前移动一格
        // 这样当我们迭代完一次数组，不为0的数字都移动到了前面，剩下的数字都为0
        // Passed 0ms 2.1mb
        let mut ptr = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, ptr);
                ptr += 1;
            }
        }
    }
}