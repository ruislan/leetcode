use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // 方法1
        // 并查集
        // 将grid[row][col] == '1'的四周的联通的'1'的放入并查集
        // 然后统计并查集的集合数量就是结果
        // Passed 8ms 5.4mb
        // fn find(mut x: usize, parent: &mut Vec<usize>) -> usize {
        //     if x != parent[x] {
        //         parent[x] = find(parent[x], parent);
        //     }
        //     parent[x]
        // }
        //
        // fn union(mut x: usize, mut y: usize, parent: &mut Vec<usize>) -> bool {
        //     let mut root_x = find(x, parent);
        //     let mut root_y = find(y, parent);
        //     if root_x == root_y { return false; }
        //     parent[root_y] = root_x;
        //     true
        // }
        //
        // fn index_of(row: usize, col: usize, cols: usize) -> usize { row * cols + col }
        //
        // let rows = grid.len();
        // let cols = grid[0].len();
        // let n = rows * cols;
        // let mut parent = vec![0; n as usize];
        // for i in 0..n as usize { parent[i] = i; }
        //
        // let mut land = std::collections::HashSet::new();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         if grid[row][col] == '1' {
        //             land.insert(index_of(row, col, cols));
        //             if row < rows - 1 && grid[row + 1][col] == '1' {
        //                 union(index_of(row, col, cols), index_of(row + 1, col, cols), &mut parent);
        //             }
        //             if col < cols - 1 && grid[row][col + 1] == '1' {
        //                 union(index_of(row, col, cols), index_of(row, col + 1, cols), &mut parent);
        //             }
        //             if row > 0 && grid[row - 1][col] == '1' {
        //                 union(index_of(row, col, cols), index_of(row - 1, col, cols), &mut parent);
        //             }
        //             if col > 0 && grid[row][col - 1] == '1' {
        //                 union(index_of(row, col, cols), index_of(row, col - 1, cols), &mut parent);
        //             }
        //         }
        //     }
        // }
        //
        // let mut island = std::collections::HashSet::new();
        // for i in 0..n {
        //     if land.contains(&i) {
        //         island.insert(find(i, &mut parent));
        //     }
        // }
        //
        // island.len() as i32

        // 方法2
        // DFS，其实很容易想到DFS，
        // grid[row][col] == '1'的四周的联通的'1'遍历直到结束
        // 然后一个visited的存储已经浏览过的'1'，
        // 这样，每次走完一次DFS，就是一个集合（岛屿）被遍历
        // 统计结果就行了
        // Passed 8ms 5.2mb
        // let rows = grid.len();
        // let cols = grid[0].len();
        // let mut answer = 0;
        // let mut visited = std::collections::HashSet::new();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         if grid[row][col] == '0' || visited.contains(&(row, col)) {
        //             continue;
        //         }
        //         answer += 1;
        //         let mut stack = vec![(row, col)];
        //         while !stack.is_empty() {
        //             let (row, col) = stack.pop().unwrap();
        //             if visited.insert((row, col)) {
        //                 if row < rows - 1 && grid[row + 1][col] == '1' {
        //                     stack.push((row + 1, col));
        //                 }
        //                 if col < cols - 1 && grid[row][col + 1] == '1' {
        //                     stack.push((row, col + 1));
        //                 }
        //                 if row > 0 && grid[row - 1][col] == '1' {
        //                     stack.push((row - 1, col));
        //                 }
        //                 if col > 0 && grid[row][col - 1] == '1' {
        //                     stack.push((row, col - 1));
        //                 }
        //             }
        //         }
        //     }
        // }
        // answer

        // 方法3
        // 可以让访问过的“1”变成“0”，这样可以节约很多搜索和hash运算
        // rust的hash很昂贵
        // Passed 8ms 5.3mb
        let rows = grid.len();
        let cols = grid[0].len();
        let mut answer = 0;
        let mut grid = grid;
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '0' { continue; }
                answer += 1;
                let mut stack = vec![(row, col)];
                while !stack.is_empty() {
                    let (row, col) = stack.pop().unwrap();
                    grid[row][col] = '0';
                    if row < rows - 1 && grid[row + 1][col] == '1' { stack.push((row + 1, col)); }
                    if col < cols - 1 && grid[row][col + 1] == '1' { stack.push((row, col + 1)); }
                    if row > 0 && grid[row - 1][col] == '1' { stack.push((row - 1, col)); }
                    if col > 0 && grid[row][col - 1] == '1' { stack.push((row, col - 1)); }
                }
            }
        }
        answer
    }
}