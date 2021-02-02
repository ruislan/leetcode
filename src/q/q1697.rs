use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // 方法1
        // 首先对edges进行排序，排序是以dis作为权重，从小到大
        // 对于每次query都用并查集连接dis小于query[2]的边，
        // 连接完了之后，如果query[1]和query[2]联通
        // 说明存在，否则说明不存在
        // 上面这样操作会在数据量很大的时候导致超时。
        // 为了避免大数据量超时，那么可以先对query进行排序
        // 这样，每次query都可以忽略前一次已经联通的部分，
        // 因为前面不管怎么联通，都小于这一次的limit
        // 这样可以避免很多重复的unite
        // AC 76ms 9.4mb
        let n = n as usize;
        let mut edges = edge_list;
        edges.sort_unstable_by(|a, b| a[2].cmp(&b[2]));

        let mut queries = queries;
        for i in 0..queries.len() { queries[i].push(i as i32); }
        queries.sort_unstable_by(|a, b| a[2].cmp(&b[2]));

        let mut answer = vec![false; queries.len()];
        let mut uf = UnionFind::new(n);
        let mut i = 0;
        for query in queries {
            while i < edges.len() && edges[i][2] < query[2] {
                uf.unite(edges[i][0] as usize, edges[i][1] as usize);
                i += 1;
            }
            answer[query[3] as usize] = uf.check_connected(query[0] as usize, query[1] as usize)
        }
        answer
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