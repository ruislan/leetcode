use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        // 方法1
        // 并查集
        // 找出所有交换过一次相等的字符进行union
        // 然后直接得到set_count就是结果
        // 注意，两个一样的字符串也是相似的
        // AC 4ms 2.2mb
        let mut n = strs.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in i + 1..n {
                let mut diff = 0;
                // 这个地方是经过了优化，优化前是：
                // strs[i].chars().zip(strs[j].chars()).filter(|&(a,b)| a != b).count()
                // 虽然lambda很美，但是这里会遍历所有的字符，很多字符其实不需要遍历完就知道结果
                // 优化前的时间是 40~92ms左右，这个差距还是挺大
                for (a, b) in strs[i].chars().zip(strs[j].chars()) {
                    if a != b { diff += 1; }
                    if diff > 2 { break; }
                }
                if diff <= 2 {
                    uf.union(i, j);
                }
            }
        }
        uf.set_count as i32
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    pub set_count: usize,
}

#[allow(unused)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind { parent: (0..n).collect(), set_count: n }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }
        self.parent[root_y] = root_x;
        self.set_count -= 1;
        true
    }
}