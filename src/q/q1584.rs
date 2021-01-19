use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 题干有“请你返回将所有点连接的最小总费用”和“任意两点之间有且仅有一条边”，
        // 首先想到的就是无环带权，也就是最短路径，最小生成树等等
        // 而“所有点连接的”这个部分告诉我们只要联通就好了，不需要形成最短路径
        // 这种情况就是典型的最小生成树，那么我们有两个算法可以选择Prim和Kruskal
        // Kruskal思想相对简单，也是贪心思想的应用之一，通过排序带权边，然后把最小的边先放入，
        // 如果当前放入的边形成了环路，就丢弃这个边，直到所有的边都放入
        // 就成了最小生成树，也就解决了我们这个题
        // 那么检查环路大声告诉我，用什么？“并查集！！！！”
        // Passed 136ms 19.5mb
        // 优化方向，并查集可以做一定的优化，生成的边edges也可以优化
        let n = points.len();
        let mut edges = Vec::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                edges.push((i, j, (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()));
            }
        }
        edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

        let mut parents = Vec::new();
        for i in 0..n { parents.push(i); }

        let mut answer = 0;
        for edge in edges {
            let mut a = edge.0;
            let mut b = edge.1;
            while a != parents[a] { a = parents[a]; }
            while b != parents[b] { b = parents[b]; }
            if a == b { continue; }
            parents[b] = a;
            answer += edge.2;
        }
        answer
    }
}