use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        // 方法1
        // 这种联通性质的检查就是并查集的打击范围了
        // 由于输入的只有小写字母，所以总数是n的并查集
        // 然后检查，如果是“==”表示两个变量联通
        // 如果是“!=”则表示两个变量不能联通
        // 这里有个小技巧就是先将所有能够联通的都联通起来
        // 然后再判断不能联通的，如果不能联通的两个变量
        // 属于同一个集合，那么则说明这个等式组是矛盾的
        // 那么返回false
        // 都检查完了，不矛盾，那么返回true
        let mut uf = UnionFind::new(26);
        let mut blocks = Vec::new();
        for equation in equations {
            let bytes = equation.into_bytes();
            if b'!' == bytes[1] {
                blocks.push(bytes);
            } else {
                uf.union(bytes[0] as usize - 97, bytes[3] as usize - 97);
            }
        }

        for block in blocks {
            if uf.find(block[0] as usize - 97) == uf.find(block[3] as usize - 97) {
                return false;
            }
        }
        true
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