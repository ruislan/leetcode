use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // 方法1
        // 从索引0开始，我们查看当前值最远能够到达的距离和已经存储的最远距离之间的最远的距离
        // 每次迭代索引都前进1
        // 索引一直前进到最远的距离位置
        // 如果最远的距离已经超过了最后一个位置，那么就说明可以到达
        // Passed 0ms 2.1mb
        let n = nums.len() - 1;
        let mut i = 0;
        let mut max = 0;
        while i <= max {
            if max >= n { return true; }
            max = max.max(i + nums[i] as usize);
            i += 1;
        }
        false
    }
}