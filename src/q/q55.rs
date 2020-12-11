use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // 方法1
        // 又是一道经典动态规划题，值得回味
        // 从索引0开始，我们查看当前值最远能够到达的距离和已经存储的最远距离之间的最远的距离
        // 每次迭代索引都前进1
        // 索引一直前进到最远的距离位置
        // 如果最远的距离已经超过了最后一个位置，那么就说明可以到达
        let mut pos = 0;
        let mut max_reach = 0;
        while pos <= max_reach {
            if max_reach >= nums.len() - 1 { return true; }
            max_reach = max_reach.max(pos + nums[pos] as usize);
            pos += 1;
        }
        false
    }
}