#[derive(PartialEq, Eq, Clone, Debug)]
struct UnionFindSet {
    sets: Vec<usize>,
    rank: Vec<usize>,
}

#[allow(unused)]
impl UnionFindSet {
    fn new(n: usize) -> Self {
        let mut uf = UnionFindSet { sets: vec![0; n + 1], rank: vec![0; n + 1] };
        uf.make_sets();
        uf
    }

    fn make_sets(&mut self) {
        for i in 0..self.sets.len() {
            self.sets[i] = i;
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut item = x;
        while item != self.sets[item] { item = self.sets[item]; }
        while x != item { // flat tree
            self.sets[x] = item;
            x = self.sets[x];
        }
        self.sets[x]
    }

    fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }
        // check rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Greater => self.sets[root_y] = root_x,
            std::cmp::Ordering::Less => self.sets[root_x] = root_y,
            _ => {
                self.sets[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        true
    }
}