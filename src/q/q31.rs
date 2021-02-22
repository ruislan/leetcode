use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 方法1
        // 从后向前搜索，找到nums[i - 1] < nums[i]的情况
        // 说明我们可以调整，然后将这两个数字交换
        // 交换之后相当于我们进了更大的数字的这个排列，
        // 所以后面的数字需要按照字典顺序排列，也就是升序
        // 我们将i..n位置的所有数字进行排序处理
        // 处理完成之后，就是下一个排列，立刻返回结果
        // 如果迭代完了，说明是最大的字典排列（倒序）
        // 那么直接翻转数组就行了
        let n = nums.len();
        for i in (1..n).rev() {
            if nums[i - 1] < nums[i] {
                nums.swap(i - 1, i);
                nums[i..].sort_unstable();
                return;
            }
        }
        nums.reverse();
    }
}