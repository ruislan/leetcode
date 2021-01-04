use crate::lcp::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        // 方法1
        // 创建map存储所有的start -> end(1,2,3,...)的路径，
        // 用path存储待搜索的起点，path的初始起点是0
        // 然后采用广度优先搜索k次
        // 每次从map中取出下一轮起点（当前起点的终点们）的位置存入path
        // 最后统计path中哪些是达到了终点(n - 1)的个数
        // Passed 0ms 2.2mb
        let mut map = std::collections::HashMap::new();

        for rel in relation {
            map.entry(rel[0]).or_insert(Vec::new()).push(rel[1]);
        }

        let mut path = std::collections::VecDeque::new();
        path.push_back(0);
        for _ in 0..k {
            for _ in 0..path.len() {
                let start = path.pop_front().unwrap();
                if let Some(v) = map.get(&start) {
                    for &new_start in v {
                        path.push_back(new_start);
                    }
                }
            }
        }

        path.into_iter().filter(|&x| x == n - 1).count() as i32
    }
}