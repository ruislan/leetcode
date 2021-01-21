use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 方法1
        // 最小生成树的方法最好用的就是kruskal
        // 关键边的判断：
        //    1.删除了这条边，集合不是一个了
        //    2.删除了这条边，权值和增加了（不再是最小生成树）
        // 伪关键边的判断：
        //    1.是最小生成树的一边
        //    2.删除了这条边，权值不变
        // 所以我们迭代edges：
        //    先判断关键边，就用上面的方式
        //    然后判断伪关键边，
        //    这里要注意的是，如果直接去看删除这个边留下的边，
        //    那么编码会变得很复杂。所以要变换一下思维。
        //    删除这条边权值不变，这里可以转换成留下这条边权值不变，
        //    也就是说删除掉其他和它等价的边都是可以的，那么这条边
        //    就是伪关键边。
        // 把握住了上述就没问题了
        // Passed 24ms 2mb
        let n = n as usize;
        let mut edges = edges;
        for i in 0..edges.len() { edges[i].push(i as i32); }
        edges.sort_unstable_by(|a, b| a[2].cmp(&b[2]));

        let mut uf = UnionFind::new(n);
        let mut min_weight = 0;
        for i in 0..edges.len() {
            if uf.union(edges[i][0] as usize, edges[i][1] as usize) {
                min_weight += edges[i][2];
            }
        }

        let mut answer = vec![Vec::new(); 2];
        for i in 0..edges.len() {
            let mut uf = UnionFind::new(n);
            let mut weight = 0;
            for j in 0..edges.len() {
                if i != j && uf.union(edges[j][0] as usize, edges[j][1] as usize) {
                    weight += edges[j][2];
                }
            }
            if uf.sets > 1 || weight > min_weight {
                answer[0].push(edges[i][3]);
                continue;
            }

            let mut uf = UnionFind::new(n);
            uf.union(edges[i][0] as usize, edges[i][1] as usize);
            let mut weight = edges[i][2];
            for j in 0..edges.len() {
                if i != j && uf.union(edges[j][0] as usize, edges[j][1] as usize) {
                    weight += edges[j][2];
                }
            }
            if weight == min_weight {
                answer[1].push(edges[i][3]);
            }
        }

        answer
    }
}


struct UnionFind {
    parent: Vec<usize>,
    sets: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n { parent[i] = i; }
        UnionFind { parent, sets: n }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y { return false; }
        self.parent[root_y] = root_x;
        self.sets -= 1;
        true
    }
}