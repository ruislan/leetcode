use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 与924的方法名重复，改成min_malware_spread_ii
    pub fn min_malware_spread_ii(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        // 方法1
        // 由于删除一个节点将完全断开所有的连接
        // 所以首先不考虑最终感染点，将其他的点都联通起来成为各个连通分量
        // 然后加入感染点，如果这个感染点使得最终的联通点数量变得更多，
        // 则这个点就是答案
        // 还只是想法，以下编码只是预实现
        let rows = graph.len();
        let cols = graph[0].len();
        let mut uf = UnionFind::new(rows);

        let mut set: std::collections::HashSet<usize> = initial.iter().map(|&node| node as usize).collect();

        for row in 0..rows {
            for col in 0..cols {
                if graph[row][col] == 1 && !set.contains(&row) && !set.contains(&col) {
                    uf.unite(row, col);
                }
            }
        }

        let mut answer = (0, 0);
        for i in 0..initial.len() {
            let mut amount = 0;
            for row in 0..rows {
                if graph[row][initial[i] as usize] == 1 && !set.contains(&row) {
                    let root = uf.find(row);
                    amount += uf.sz[root];
                }
            }
            amount += 1;
            if answer.0 < amount {
                answer.0 = amount;
                answer.1 = initial[i];
            }
        }
        answer.1
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    sz: Vec<usize>,
    pub set_count: usize,
}

#[allow(unused)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), rank: vec![0; n], sz: vec![1; n], set_count: n }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.sz[root_x] += self.sz[root_y];
            }
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.sz[root_y] += self.sz[root_x];
            }
            _ => {
                self.parent[root_y] = root_x;
                self.sz[root_x] += self.sz[root_y];
                self.rank[root_x] += 1;
            }
        }
        self.set_count -= 1;
        true
    }

    pub fn check_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}