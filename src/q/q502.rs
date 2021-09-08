use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        // 方法1
        // 要使得我们的w最多，
        // 如果我们的w够选择项目，那么我们就选择净利润最高的，这就是一个贪心逻辑了
        // 这里我们有两个要素，一个是启动项目的资本capital和利润profits
        // 我们当然希望的是资本尽可能的小，利润尽可能的大，
        // 所以我们先排序资本，然后再将可以选的项目放进去被选项，然后找最大利润的
        // 例如： p:[1 2 3] c:[0 1 1]，w=0,k=2
        // 这里的w=0，我们只能找到c=0的，也就是1，我们得到了利润1，
        // 然后我们可以选择两个了，利润(1,2)和（1，3），所以我们肯定选3了
        // 最后我们的资本就是0 + 1 + 3 = 4
        // P.S 我刚刚开始做的时候，以为还要扣除capital的值，后来发现profit就是净利润（也就是已经回本后的，所以不用减去了）
        // AC 20ms 4.8mb 35/35
        let mut projects = 0;
        let mut w = w;
        let mut pair: Vec<(i32, i32)> = capital.iter().zip(profits.iter()).map(|(&a, &b)| (a, b)).collect();
        pair.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut heap = std::collections::BinaryHeap::new();
        let mut i = 0;
        while projects < k {
            while i < pair.len() && w >= pair[i].0 {
                heap.push(pair[i].1);
                i += 1;
            }
            if heap.is_empty() { break; }
            w += heap.pop().unwrap();
            projects += 1;
        }
        w
    }
}

