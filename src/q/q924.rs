use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        // 方法1
        // 将所有的点联通起来
        // 删除某个点之后，那个点所在的集合数量最多，那个就是答案
        // 还只是想法，以下编码只是预实现
        let rows = graph.len();
        let cols = graph[0].len();
        let mut uf = UnionFind::new(rows);
        for row in 0..rows {
            for col in 0..cols {
                if graph[row][col] == 1 {
                    uf.unite(row, col);
                }
            }
        }
        let mut initial = initial;
        initial.sort_unstable();
        let mut answer = (0, 0);
        for i in 0..initial.len() {
            // let root = uf.find(initial[i] as usize);
            // let sz = uf.sz[root];
            let sz = uf.sz[initial[i] as usize];
            if answer.0 < sz {
                answer.0 = sz;
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