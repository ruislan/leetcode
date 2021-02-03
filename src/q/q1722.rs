use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 将所有可以交换的看成一个联通的网络
        // 在这个网络中所有的元素都是可以自由放在任意位置的
        // 联通之后就需要用hashmap存储source的元素的原始位置
        // 当我们使用了一次替换后，就不能再使用这个数字了
        // 也就是说，我们遇到不同那么就开始替换，能替换就减少网络中的一个可替换量
        // 不能替换的，当然就结果+1
        // 当然，如果这个数字在source中就没有出现过，那么也+1
        // AC 76ms 15.3mb
        let n = source.len();
        let mut uf = UnionFind::new(n);

        for allow in allowed_swaps {
            uf.unite(allow[0] as usize, allow[1] as usize);
        }
        let mut indices = std::collections::HashMap::new();
        for i in 0..n {
            indices.entry(source[i]).or_insert(Vec::new()).push(i);
        }
        let mut used = vec![false; n];
        let mut answer = 0;
        for i in 0..n {
            let other = indices.get(&target[i]);
            let mut check = || -> bool {
                let v = other.unwrap();
                for &j in v.iter() {
                    if !used[j] && uf.check_connected(i, j) {
                        used[j] = true;
                        return true;
                    }
                }
                false
            };
            if other.is_none() || !check() {
                answer += 1;
            }
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