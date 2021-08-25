use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 方法1
        // DFS就可以了，这个题算是DFS的教科书题了
        // AC 4ms 2.6mb
        fn dfs(path: &mut Vec<i32>, i: usize, end: usize, graph: &Vec<Vec<i32>>, answer: &mut Vec<Vec<i32>>) {
            if i == end {
                answer.push(path.clone());
                return;
            }
            for &j in graph[i].iter() {
                path.push(j);
                dfs(path, j as usize, end, graph, answer);
                path.pop();
            }
        }
        let mut answer = Vec::new();
        dfs(&mut vec![0], 0, graph.len() - 1, &graph, &mut answer);
        answer
    }
}