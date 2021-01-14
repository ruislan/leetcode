use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 用图画一下石头的摆放，例如：
        //    0   1   2
        // 0  x - x
        // 1  x --|-- x
        // 2      x-- x
        // 我们可以看出来，如果所有的石头连在一起
        // 那么结果就是连在一起的石头 - 1
        // 换而言之也就是总的石头数n - 可以连在一起的石头数(count)
        // 要把所有的合格石头连在一起，
        // 就只需要检查每个石头的每一行和每一列
        // 将他们一一加入到联通的石头中去
        // 所以并查集就是一个很好的处理这样的事情的数据结构
        // Passed 24ms 2.2mb （这里给的是最直接的解决方法，还有优化空间）
        let mut n = stones.len();
        let mut size = vec![0; n];
        let mut parents = vec![0; n];
        for i in 0..n { parents[i] = i; }
        let mut count = n;

        for i in 0..n {
            for j in i + 1..n {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    let mut a = i;
                    let mut b = j;
                    while a != parents[a] { a = parents[a]; }
                    while b != parents[b] { b = parents[b]; }
                    if a == b { continue; }
                    if size[a] > size[b] {
                        parents[b] = a;
                        size[a] += 1;
                    } else {
                        parents[a] = b;
                        size[b] += 1;
                    }
                    count -= 1;
                }
            }
        }

        (n - count) as i32
    }
}