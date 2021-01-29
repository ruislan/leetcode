use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 利用最小生成树的思想解决
        // 将所有的带权边都求出来
        // 然后将带权边排序（Kruskal）
        // 然后从最小的带权边开始联通，
        // 直到我们将0,0和rows - 1,cols - 1刚好联通
        // 这个刚好联通的带权边的权就是最小的那个权
        // AC 24ms 2.9mb
        let rows = heights.len();
        let cols = heights[0].len();
        let index_of = |row: usize, cols: usize, col: usize| -> usize{ row * cols + col };
        let mut uf = UnionFind::new(rows * cols);
        let mut edges = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                let height = index_of(row, cols, col);
                if col > 0 {
                    let left = index_of(row, cols, col - 1);
                    let diff = (heights[row][col] - heights[row][col - 1]).abs();
                    edges.push((diff, left, height));
                }
                if row > 0 {
                    let up = index_of(row - 1, cols, col);
                    let diff = (heights[row][col] - heights[row - 1][col]).abs();
                    edges.push((diff, up, height));
                }
            }
        }
        edges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let end = rows * cols - 1;
        for edge in edges {
            uf.union(edge.1, edge.2);
            if uf.check_connected(0, end) {
                return edge.0;
            }
        }
        0
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    sz: Vec<usize>,
    pub set_count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n { parent[i] = i; }
        UnionFind { parent, rank: vec![0; n], sz: vec![1; n], set_count: n }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
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
                self.rank[root_x] += 1;
                self.sz[root_x] += self.sz[root_y];
            }
        }
        self.set_count -= 1;
        true
    }

    pub fn check_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}