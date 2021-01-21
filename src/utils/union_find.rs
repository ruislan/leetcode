#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    pub set_count: usize,
}

#[allow(unused)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n { parent[i] = i; }
        let mut uf = UnionFind { parent, rank: vec![0; n], set_count: n };
        uf
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        let mut item = x;
        while item != self.parent[item] { item = self.parent[item]; }
        while x != item { // flat tree
            self.parent[x] = item;
            x = self.parent[x];
        }
        self.parent[x]
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }
        // check rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            std::cmp::Ordering::Less => self.parent[root_x] = root_y,
            _ => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        self.set_count -= 1;
        true
    }

    pub fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}