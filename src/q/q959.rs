use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        // 方法1
        // 将单个方格划分为4个三角，
        //          \ base + 1 /
        //  base + 0      X      base + 2
        //          / base + 3 \
        // 而base通过(row * n + col) * 4得到（也就是第N个方格的初始值）
        // 然后每个方格：
        //    左三角和前一个方格的右三角联通
        //    右三角和后一个方格的左三角联通
        //    上三角和上一个方格的下三角联通
        //    下三角和下一个方格的上三角联通
        // 然后利用并查集，将所有联通的都联通起来
        // 然后集合的数量就是结果
        // Passed 4ms 2.1mb
        let n = grid.len();
        let mut uf = UnionFind::new(n * n * 4);
        for (row, item) in grid.into_iter().enumerate() {
            let chars: Vec<char> = item.chars().collect();
            for col in 0..n {
                let base = (row * n + col) * 4;
                match chars[col] {
                    '/' => {
                        uf.union(base, base + 1);
                        uf.union(base + 2, base + 3);
                    }
                    '\\' => {
                        uf.union(base, base + 3);
                        uf.union(base + 1, base + 2);
                    }
                    _ => {
                        uf.union(base, base + 1);
                        uf.union(base + 1, base + 2);
                        uf.union(base + 2, base + 3);
                    }
                }
                if row < n - 1 { uf.union(base + 3, ((row + 1) * n + col) * 4 + 1); }
                if row > 0 { uf.union(base + 1, ((row - 1) * n + col) * 4 + 3); }
                if col < n - 1 { uf.union(base + 2, (row * n + col + 1) * 4); }
                if col > 0 { uf.union(base, (row * n + col - 1) * 4 + 2); }
            }
        }
        uf.set_count as i32
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    sz: Vec<usize>,
    pub set_count: usize,
}

#[allow(unused)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n { parent[i] = i; }
        let mut uf = UnionFind { parent, rank: vec![0; n], sz: vec![1; n], set_count: n };
        uf
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