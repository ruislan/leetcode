use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 设置一共有N个砖块
        // 打掉hits[i]个方块之后，检查所有0行联通的砖块有多少个
        // 然后(n - 1) - check_connected(0行)就是当前删除了多少个砖块
        // NotPassed 大数据下超时
        // let check_connected = |grid: &mut Vec<Vec<i32>>, rows: usize, cols: usize| -> i32 {
        //     let mut count = 0;
        //     let mut visited = std::collections::HashSet::new();
        //     for col in 0..cols {
        //         if grid[0][col] == 1 && !visited.contains(&(0, col)) {
        //             let mut stack = vec![(0, col)];
        //             while !stack.is_empty() {
        //                 let (row, col) = stack.pop().unwrap();
        //                 if visited.insert((row, col)) {
        //                     count += 1;
        //                     if row > 0 && grid[row - 1][col] == 1 { stack.push((row - 1, col)); }
        //                     if row < rows - 1 && grid[row + 1][col] == 1 { stack.push((row + 1, col)); }
        //                     if col > 0 && grid[row][col - 1] == 1 { stack.push((row, col - 1)); }
        //                     if col < cols - 1 && grid[row][col + 1] == 1 { stack.push((row, col + 1)); }
        //                 }
        //             }
        //         }
        //     }
        //     for row in 0..rows {
        //         for col in 0..cols {
        //             grid[row][col] = 0;
        //         }
        //     }
        //     for (row, col) in visited {
        //         grid[row][col] = 1;
        //     }
        //     count
        // };
        //
        // let mut answer = Vec::new();
        // let mut n = 0;
        // let mut grid = grid;
        // let mut rows = grid.len();
        // let mut cols = grid[0].len();
        // for row in 0..rows {
        //     for col in 0..cols {
        //         if grid[row][col] == 1 {
        //             n += 1;
        //         }
        //     }
        // }
        //
        // for hit in hits {
        //     let row = hit[0] as usize;
        //     let col = hit[1] as usize;
        //     if n == 0 || grid[row][col] == 0 {
        //         answer.push(0);
        //     } else {
        //         grid[row][col] = 0;
        //         let new_n = check_connected(&mut grid, rows, cols);
        //         answer.push(n - 1 - new_n);
        //         n = new_n;
        //     }
        // }
        // answer

        // 方法2
        // 与方法1不同，我们用dfs的方式检查删除节点的4个方向上的节点各自的联通性是否含有顶部的联通
        // 如果有，不会掉落，如果没有，就全部都掉落
        // Not Passed， 可以看出来基于dfs的大数据下依然超时,所以我们要换个思路
        // let dfs_check_fall = |grid: &mut Vec<Vec<i32>>, rows: usize, cols: usize, x: (usize, usize)| -> i32 {
        //     let mut visited = std::collections::HashSet::new();
        //     let mut stack = vec![x];
        //     let mut fall = true;
        //     while !stack.is_empty() {
        //         let (row, col) = stack.pop().unwrap();
        //         if visited.insert((row, col)) {
        //             if row == 0 {
        //                 fall = false;
        //                 break;
        //             }
        //             if row > 0 && grid[row - 1][col] == 1 { stack.push((row - 1, col)); }
        //             if row < rows - 1 && grid[row + 1][col] == 1 { stack.push((row + 1, col)); }
        //             if col > 0 && grid[row][col - 1] == 1 { stack.push((row, col - 1)); }
        //             if col < cols - 1 && grid[row][col + 1] == 1 { stack.push((row, col + 1)); }
        //         }
        //     }
        //     if fall {
        //         for visit in visited.iter() {
        //             grid[visit.0][visit.1] = 0;
        //         }
        //         visited.len() as i32
        //     } else {
        //         0
        //     }
        // };
        //
        // let mut answer = Vec::new();
        // let mut grid = grid;
        // let mut rows = grid.len();
        // let mut cols = grid[0].len();
        //
        // for i in 0..hits.len() {
        //     let row = hits[i][0] as usize;
        //     let col = hits[i][1] as usize;
        //     if grid[row][col] == 0 {
        //         answer.push(0);
        //     } else {
        //         grid[row][col] = 0;
        //         let mut count = 0;
        //         if row > 0 && grid[row - 1][col] == 1 { count += dfs_check_fall(&mut grid, rows, cols, (row - 1, col)); }
        //         if row < rows - 1 && grid[row + 1][col] == 1 { count += dfs_check_fall(&mut grid, rows, cols, (row + 1, col)); }
        //         if col > 0 && grid[row][col - 1] == 1 { count += dfs_check_fall(&mut grid, rows, cols, (row, col - 1)); }
        //         if col < cols - 1 && grid[row][col + 1] == 1 { count += dfs_check_fall(&mut grid, rows, cols, (row, col + 1)); }
        //         answer.push(count);
        //     }
        // }
        // answer

        // 方法3
        // 利用并查集，将分割的集合链接起来
        // 第一步：先将所有的hits点都删除掉，这样剩下的点各自先抱成团（生成并查集合）
        //    细节1：二维要变一维数组，长度就是rows * cols，同时还要多一个(+1)作为边缘或者顶点
        // 第二步：逆序迭代恢复hits，因为如果顺序去恢复的话，有些集合可能因为缺少连接点（后面的hits）而没有链接起来
        //    细节1：在恢复点的时候，注意如果是第0行，那么还得把这个点先和顶点合并
        //    细节2：在恢复点的时候，要查找其上下左右，并与之做合并
        //    细节3：在恢复点之前，要先知道顶点联通的点的总的数量，恢复之后得到总的数量，
        //          这个两个数量之差再减去这个点自己就是当前掉落的点的数量
        // Passed 72ms 4.1mb
        fn init(parents: &mut Vec<usize>, size: &mut Vec<usize>, n: usize) {
            for i in 0..n {
                parents[i] = i;
                size[i] = 1;
            }
        }
        fn find(parents: &mut Vec<usize>, mut x: usize) -> usize {
            if x != parents[x] {
                parents[x] = find(parents, parents[x]);
            }
            parents[x]
        }
        fn union(parents: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
            let mut root_x = find(parents, x);
            let mut root_y = find(parents, y);
            if root_x == root_y { return; }
            parents[root_x] = root_y;
            size[root_y] += size[root_x];
        }
        fn get_size(parents: &mut Vec<usize>, size: &mut Vec<usize>, x: usize) -> usize {
            size[find(parents, x)]
        }

        fn index_of(row: usize, col: usize, cols: usize) -> usize { row * cols + col }

        let mut rows = grid.len();
        let mut cols = grid[0].len();
        let mut size = rows * cols;
        let mut parents = vec![0; size + 1];
        let mut sz = vec![0; size + 1];
        let mut grid = grid;
        let mut real_hits = vec![false; hits.len()];
        for i in 0..hits.len() {
            let row = hits[i][0] as usize;
            let col = hits[i][1] as usize;
            if grid[row][col] == 1 { real_hits[i] = true; }
            grid[row][col] = 0;
        }

        // 初始化并查集
        init(&mut parents, &mut sz, size + 1);

        // 设置第0排的和顶部union
        for col in 0..cols {
            if grid[0][col] == 1 { union(&mut parents, &mut sz, col, size); }
        }

        // 其他砖块如果和0排相连，就union
        for row in 1..rows {
            for col in 0..cols {
                if grid[row][col] == 1 {
                    if grid[row - 1][col] == 1 { // 上方相邻union
                        union(&mut parents, &mut sz, index_of(row - 1, col, cols), index_of(row, col, cols));
                    }
                    if col > 0 && grid[row][col - 1] == 1 { // 左边相邻union
                        union(&mut parents, &mut sz, index_of(row, col - 1, cols), index_of(row, col, cols));
                    }
                }
            }
        }
        let mut answer = vec![0; hits.len()];
        for i in (0..hits.len()).rev() {
            let row = hits[i][0] as usize;
            let col = hits[i][1] as usize;

            if !real_hits[i] { continue; }

            let before = get_size(&mut parents, &mut sz, size);

            if row == 0 { union(&mut parents, &mut sz, col, size); }

            let node = index_of(row, col, cols);
            if col < cols - 1 && grid[row][col + 1] == 1 { union(&mut parents, &mut sz, node, index_of(row, col + 1, cols)); }
            if row < rows - 1 && grid[row + 1][col] == 1 { union(&mut parents, &mut sz, node, index_of(row + 1, col, cols)); }
            if col > 0 && grid[row][col - 1] == 1 { union(&mut parents, &mut sz, node, index_of(row, col - 1, cols)); }
            if row > 0 && grid[row - 1][col] == 1 { union(&mut parents, &mut sz, node, index_of(row - 1, col, cols)); }

            let current = get_size(&mut parents, &mut sz, size);
            answer[i] = 0.max(current as i32 - before as i32 - 1);

            grid[row][col] = 1;
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]], vec![vec![1, 0]]), vec![2]);
    assert_eq!(Solution::hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]], vec![vec![1, 1], vec![1, 0]]), vec![0, 0]);
    assert_eq!(Solution::hit_bricks(vec![vec![1, 0, 1], vec![1, 1, 1]], vec![vec![0, 0], vec![0, 2], vec![1, 1]]), vec![0, 3, 0]);
    assert_eq!(Solution::hit_bricks(vec![vec![1], vec![1], vec![1], vec![1], vec![1]], vec![vec![3, 0], vec![4, 0], vec![1, 0], vec![2, 0], vec![0, 0]]), vec![1, 0, 1, 0, 0]);
}