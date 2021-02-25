use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 方法1
        // 从后向前搜索，找到nums[i] < nums[i + 1]的情况
        // 说明我们可以调整，然后从i + 1到n - 1之间找到刚好比i大的那个数字
        // 就是我们要交换的数字，交换之后相当于我们进了更大的数字的这个排列中的最小排列，
        // 也就是i + 1后面全部都是升序
        // 所以后面的数字需要按照字典顺序排列，也就是升序
        // 我们将i + 1位置后面的所有数字进行翻转就可以了
        // 处理完成之后，就是下一个排列，立刻返回结果
        // 如果迭代完了，说明是最大的字典排列（倒序）
        // 那么直接翻转数组就行了
        // AC 0ms 2mb
        let n = nums.len();
        for i in (0..n - 1).rev() {
            if nums[i] < nums[i + 1] {
                let mut swap_at = n - 1;
                while nums[i] >= nums[swap_at] { swap_at -= 1; }
                nums.swap(i, swap_at);
                nums[i + 1..].reverse();
                return;
            }
        }
        nums.reverse();
    }
}