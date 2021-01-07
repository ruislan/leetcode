use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 构造一个有向图graph
        // 迭代n个城市，1..=n，
        // 先检查是否已经访问过了：
        //    如果没有，按照广度或者深度优先探索，并将探索过的放入访问过的，
        //    以免循环访问，一直到结束，就形成了一个省
        // 统计省并返回结果
        // Passed 4ms 2.2mb
        // let mut graph = std::collections::HashMap::new();
        // let n = is_connected.len();
        // for row in 0..n {
        //     for col in 0..n {
        //         if row != col && is_connected[row][col] == 1 {
        //             graph.entry(row).or_insert(Vec::new()).push(col);
        //         }
        //     }
        // }
        //
        // let mut answer = 0;
        // let mut visited = std::collections::HashSet::new();
        // for city in 0..n {
        //     if visited.insert(city) {
        //         let mut stack = vec![city];
        //         while !stack.is_empty() {
        //             let cur_city = stack.pop().unwrap();
        //             if let Some(next_cities) = graph.get(&cur_city) {
        //                 for &next_city in next_cities {
        //                     if visited.insert(next_city) {
        //                         stack.push(next_city);
        //                     }
        //                 }
        //             }
        //         }
        //         answer += 1;
        //     }
        // }
        // answer

        // 方法2
        // 优化方法1
        // 其实也可以不用构建图，因为is_connected就是一个图了
        // Passed 0ms 2.2mb
        // let n = is_connected.len();
        // let mut answer = 0;
        // let mut visited = std::collections::HashSet::new();
        // for city in 0..n {
        //     if visited.insert(city) {
        //         let mut stack = vec![city];
        //         while !stack.is_empty() {
        //             let cur_city = stack.pop().unwrap();
        //             for (next_city, &connect) in is_connected[cur_city].iter().enumerate() {
        //                 if connect == 1 && visited.insert(next_city) {
        //                     stack.push(next_city);
        //                 }
        //             }
        //         }
        //         answer += 1;
        //     }
        // }
        // answer

        // 方法3
        // 优化方法2
        // 也可以不用hashset，直接用数组，因为就200个城市
        // Passed 0ms 2.2mb
        let n = is_connected.len();
        let mut answer = 0;
        let mut visited = vec![false; 200];
        for city in 0..n {
            if !visited[city] {
                visited[city] = true;
                let mut stack = vec![city];
                while !stack.is_empty() {
                    let cur_city = stack.pop().unwrap();
                    for (next_city, &connect) in is_connected[cur_city].iter().enumerate() {
                        if connect == 1 && !visited[next_city] {
                            visited[next_city] = true;
                            stack.push(next_city);
                        }
                    }
                }
                answer += 1;
            }
        }
        answer
    }
}