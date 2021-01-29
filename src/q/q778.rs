use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // kruskal最小生成树的方法
        // 这里我们将联通边的权设定为两个点中大的那个存储到edges
        // 然后一切就按照kruskal的模式来处理了
        // 排序edges之后，逐渐加入edges[i]，
        // 直到(0,0)到(rows-1,cols-1)联通起来的那条边的权重就是结果
        // AC 4ms 2.1mb
        // let mut rows = grid.len();
        // let mut cols = grid[0].len();
        // let index_of = |row: usize, col: usize| -> usize { row * cols + col };
        // let mut edges = Vec::new();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         let idx = index_of(row, col);
        //         if row > 0 {
        //             edges.push((grid[row - 1][col].max(grid[row][col]), index_of(row - 1, col), idx));
        //         }
        //         if col > 0 {
        //             edges.push((grid[row][col - 1].max(grid[row][col]), index_of(row, col - 1), idx));
        //         }
        //     }
        // }
        //
        // edges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        // let mut uf = UnionFind::new(rows * cols);
        // for edge in edges {
        //     uf.union(edge.1, edge.2);
        //     if uf.check_connected(0, rows * cols - 1) {
        //         return edge.0;
        //     }
        // }
        // 0

        // 方法2
        // 迪克斯特拉算法找到最短路径
        // 本题没有负权重的情况，所以可以采用迪克斯特拉算法
        // 迪克斯特拉算法有4步
        // 1. 找到开销最低的那个节点
        // 2. 然后更新它到达所有邻居的节点的开销
        // 3. 重复1-2，直到所有的节点都处理过了
        // 4. 找到最短路径
        // 这里，我们不需要找到最短路径，我们只需要直到终点的开销就行了
        // AC 0~4ms 2mb
        let mut rows = grid.len();
        let mut cols = grid[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut costs = vec![vec![i32::MAX; cols]; rows];
        costs[0][0] = grid[0][0];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(Node { row: 0, col: 0, val: grid[0][0] }));

        while !heap.is_empty() {
            let node = heap.pop().unwrap().0;
            if visited[node.row][node.col] { continue; }
            visited[node.row][node.col] = true;
            if (node.row, node.col) == (rows - 1, cols - 1) { break; }
            let mut update_node_cost = |row: usize, col: usize| {
                let new_cost = grid[row][col].max(costs[node.row][node.col]);
                if new_cost < costs[row][col] {
                    costs[row][col] = new_cost;
                    heap.push(std::cmp::Reverse(Node { row, col, val: grid[row][col] }));
                }
            };
            if node.row > 0 { update_node_cost(node.row - 1, node.col); }
            if node.col > 0 { update_node_cost(node.row, node.col - 1); }
            if node.row < rows - 1 { update_node_cost(node.row + 1, node.col); }
            if node.col < cols - 1 { update_node_cost(node.row, node.col + 1); }
        }
        costs[rows - 1][cols - 1]
    }
}

#[derive(Eq)]
pub struct Node {
    row: usize,
    col: usize,
    val: i32,
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
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

    pub fn union(&mut self, x: usize, y: usize) -> bool {
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