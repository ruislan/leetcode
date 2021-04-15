use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 方法1
        // 这题和股票买卖很相似
        // 当然，这都是动态规划的典型题
        // 股票是当天买或者卖的收益
        // 而这题是当前这家偷和不偷的收益，
        // 也就是说，我们只需要积累两个状态，一个偷，一个不偷
        // 是否偷这一家的条件就是之前不偷的情况 + 当前这家的收益和之前偷了的收益对比
        // 谁大就留下谁
        // 例如：    1               2               3               1
        //   偷：0  (0+1).max(0)=1  (0+2).max(1)=2  (1+3).max(2)=4  (2+1).max(4)=4
        // 不偷：0   0               1               2               4
        // 这个例子，最大就是4
        // AC 0ms 2mb
        let n = nums.len();
        let mut rob = 0;
        let mut unrob = 0;
        for i in 0..n {
            let pre_rob = rob;
            rob = rob.max(unrob + nums[i]);
            unrob = pre_rob;
        }
        rob
    }
}