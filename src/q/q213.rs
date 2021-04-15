use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 方法与q198重名，改为rob_ii
    pub fn rob_ii(nums: Vec<i32>) -> i32 {
        // 方法1
        // 这道题是q198的小进阶
        // 条件中加了一个限制，就是最后一个房子和第一个房子是挨着的，
        // 所以第一个房子用了，就不能用最后一个房子，反之亦然
        // 那么其实我们可以看成两次不同处理
        // 第一次，我们从房子1到房子n-1，也就是不采用最后一个房子
        // 第二次，我们从房子2到房子n，也就是不采用第一个房子
        // 然后我们来比较两次当中哪次的利益是最大的，就是结果
        // 而每一次的过程就和q198是一样的了
        // 那简单来说就是分为偷或者不偷两个状态
        // 然后偷的状态取决于之前不偷和当前偷的和 与 之前偷的最大值
        // 不偷的状态就是之前偷的状态转移过来
        // AC 0ms 2mb
        fn calculate(nums: &Vec<i32>, from: usize, to: usize) -> i32 {
            let mut act = 0;
            let mut hold = 0;
            for i in from..to {
                let pre_act = act;
                act = act.max(hold + nums[i]);
                hold = pre_act;
            }
            act
        }
        let n = nums.len();
        if n < 2 { return nums[0]; }
        calculate(&nums, 0, n - 1).max(calculate(&nums, 1, n))
    }
}