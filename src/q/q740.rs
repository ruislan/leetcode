use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // 方法1
        // 先将所有的数字都排序并收束到数组中，
        // 我们依次选取数字，能够得到并追踪最后的得分，并保持最大的得分
        // 这里我们可以看出来，实际上当前这个数字只有选择和不选择两个状态
        // 和偷家那个题的当前这家偷和不偷，以及股票那个题当日买和不买类似了
        // 这个数字选和不选：
        //    如果选，那么我们可以获得这个数字所有的分数，因为选了一次这个数字，+-1数字都被删除了
        //    如果不选，那么我们只能不计算当前这个数字，而不能确定+-1数字是否入选
        // 动态规划的部分，其实很简单，
        // 当我们到第i个数字的时候，这个数字会有两个状态，选或不选，设置为pick和ban，
        // 选的话，那么他的前一个i-1必然不能选，不选的话，那么他的前一个i-1必然选（不然怎么得到最大分数呢）
        // 所以我们是否选择当前这个取决于，max(ban + nums[i], pick)，谁大
        // 然后如果选了当前这个(pick)，那么之前不选的那个自然要转换状态到选取这个之前的选取状态（pre_pick)
        // AC 0ms 2mb
        let mut n = *nums.iter().max().unwrap() as usize;
        let mut arr = vec![0; n + 1];
        for i in 0..nums.len() {
            arr[nums[i] as usize] += 1;
        }

        let mut pick = 0;
        let mut ban = 0;
        for i in 0..arr.len() {
            let pre_pick = pick;
            pick = pick.max(ban + arr[i] * i);
            ban = pre_pick;
        }
        pick as i32
    }
}