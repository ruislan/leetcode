use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        // 方法1
        // 这道题的重点在于：我们不知道庄家会选哪个数字
        // 所以我们必须假设，我们猜任何数字，庄家都会让我们去到最坏的情况
        // 那么，在1..n之间，假设我们猜的数字是k，
        // 这个时候，1..n会被分成了 1..k-1 k k + 1..n-1 3个部分
        // 我们把k的左右部分分别叫lower和higher吧，
        // 这里，如果k是答案，那就没问题
        // 如果在lower或者higher，那么我们就又要去选一个数字，也就是再做一次
        // 这是不是就是Divide And Conquer的感觉了
        // 那么递归是肯定需要的了，所以我们其实是要pay的就是k + max(rec(lower), rec(higher))
        // 这个值呢，对于我们自己来说，我们肯定想要的是最小的，这样我们可以pay最少，
        // 所以，我们如果发现了下面的那个情况比之前已经做过的情况还要大的，那我们就减枝
        // 最后留下的，就是最坏打算下的最少的pay
        // 这里可能会有超时的可能，所以我们需要一个mem来记录一下
        // AC 44ms 2.2mb 27/27
        fn dac(lo: usize, hi: usize, memo: &mut Vec<Vec<usize>>) -> usize {
            if lo >= hi { return 0; }
            if memo[lo][hi] > 0 { return memo[lo][hi]; }
            let mut ans = usize::MAX;
            for k in lo..=hi {
                let cur = k + dac(lo, k - 1, memo).max(dac(k + 1, hi, memo));
                ans = ans.min(cur);
            }
            memo[lo][hi] = ans;
            return ans;
        }
        let n = n as usize;
        dac(1, n, &mut vec![vec![0; n + 1]; n + 1]) as i32
    }
}