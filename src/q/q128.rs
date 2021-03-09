use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 方法1
        // 先去重再排序
        // 然后计算连续的即可
        // 但是时间是O(nlogn)
        // AC 0ms 2.3mb
        // if nums.is_empty() { return 0; }
        // let mut nums = nums;
        // nums.sort_unstable();
        // let mut answer = 1;
        // let mut count = 1;
        // for i in 1..nums.len() {
        //     if nums[i - 1] == nums[i] { continue; }
        //     if nums[i - 1] + 1 == nums[i] {
        //         count += 1;
        //     } else {
        //         count = 1;
        //     }
        //     answer = answer.max(count);
        // }
        // answer

        // 方法2
        // 并查集
        // 先对数组去重
        // 连续的就是一个集合，我们只需要找出nums[i] + 1与nums[i]相连即可
        // 当然nums[i] + 1要在数组中
        // 其实Rust排序去重是很快的，不过不满足题目的O(n)，我们也可以不排序，
        // 直接用sets去重，就是O(n)
        // 然后用map保存nums每个数字的索引，也是O(n)
        // 然后用并查集来进行union，平均也是O(n)
        // 就都是O(n)了
        // 0ms 2.6mb
        if nums.is_empty() { return 0; }
        let mut sets = std::collections::HashSet::new();
        for x in nums { sets.insert(x); }
        let mut nums: Vec<i32> = sets.into_iter().collect();
        let n = nums.len();
        let mut map = std::collections::HashMap::new();
        for i in 0..n { map.insert(nums[i], i); }
        let mut uf = UnionFind::new(n);
        for i in 0..n {
            if let Some(&j) = map.get(&(nums[i] + 1)) {
                uf.unite(i, j);
            }
        }
        *uf.sz.iter().max().unwrap() as i32
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
}