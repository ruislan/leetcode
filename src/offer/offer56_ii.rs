use crate::offer::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // 其他数字都出现了3次，一个数字只出现了1次
        // 本题没有空间限制，所以我们可以使用hashmap保存数字
        // 然后找到那个只出现了一次的数字

        // 方法2
        // rust中，排序是很快的，反而hashmap计算量比排序还慢
        // 所以这里我们可以对数组进行排序
        // 然后线性查找那个只出现了1次的数字
        0
    }
}