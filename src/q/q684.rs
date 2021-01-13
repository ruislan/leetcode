use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 利用并查集进行成环检测，成环的不进入并集，直到检查出最后一个成环的edges即可
        // 这里重点是考察并查集的使用，
        // 下面是并查集的朴素写法（不带路径压缩）
        // Passed 0ms 2mb
        // let find_root = |mut node: usize, parents: &Vec<usize>| -> usize {
        //     while node != parents[node] { node = parents[node]; }
        //     node
        // };
        // let union_set = |x: usize, y: usize, parents: &mut Vec<usize>| -> bool {
        //     let root_x = find_root(x, parents);
        //     let root_y = find_root(y, parents);
        //     if root_x == root_y { return false; }
        //     parents[root_y] = root_x;
        //     true
        // };
        // let n = edges.len();
        // let mut parents = vec![0; n + 1];
        // (1..=n).for_each(|i| parents[i] = i);
        // let mut answer = 0;
        // for i in 0..n {
        //     if !union_set(edges[i][0] as usize, edges[i][1] as usize, &mut parents) {
        //         answer = i;
        //     }
        // }
        // edges[answer].clone()

        // 方法2
        // 并查集用得熟悉后的写法
        // Passed 0ms 2mb
        let n = edges.len();
        let mut parents = vec![0; n + 1];
        (1..=n).for_each(|i| parents[i] = i);
        let mut answer = 0;
        for i in 0..n {
            let mut a = edges[i][0] as usize;
            let mut b = edges[i][1] as usize;
            while parents[a] != a { a = parents[a]; }
            while parents[b] != b { b = parents[b]; }
            if a == b { answer = i; } else { parents[b] = a; }
        }
        edges[answer].clone()
    }
}