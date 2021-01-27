use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 创建两个uf，Alice和Bob
        // 先保证type3的进入uf，因为type3能保证两个都可使用
        // 然后各自的type进入各自的uf，
        // 保存不能进入uf的边的数量
        // 如果两个uf的set大于1个，表示有一个不能遍历图，返回-1
        // 否则，返回边的数量
        // Passed 60ms 9.7mb
        let n = n as usize;
        let mut edges = edges;
        let mut uf_alice = UnionFind::new(n);
        let mut uf_bob = UnionFind::new(n);
        let mut answer = 0;

        edges.sort_unstable_by(|a, b| b[0].cmp(&a[0]));
        for edge in edges {
            let (t, u, v) = (edge[0], edge[1] as usize - 1, edge[2] as usize - 1);
            if t == 3 {
                if !uf_alice.union(u, v) || !uf_bob.union(u, v) { answer += 1; }
            } else if t == 1 {
                if !uf_alice.union(u, v) { answer += 1; }
            } else {
                if !uf_bob.union(u, v) { answer += 1; }
            }
        }
        if uf_alice.set_count > 1 || uf_bob.set_count > 1 { return -1; }
        answer
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