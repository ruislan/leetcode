use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        // 方法1
        // 球每次可以走上下左右四个方向，也就是x+1,x-1,y+1和y-1
        // 我们通过回溯或者是深度搜索可以探查每走一步的情况
        // 如果出界了，那么就符合要求，结果+1
        // 如果步数走完了，那么直接结束搜索
        // 当然，这样带来的运算量会很大，所以我们需要记忆化来优化
        // 这就是典型的dp题了
        // 我们将每个点(x,y)和已经走了的步数s做一个元组作为key来记忆这个位置的这个步数有多少种出界的可能
        // 这样就能极大的优化过程，快速求出结果
        // AC 8ms 2.9mb
        use std::collections::HashMap;
        fn dfs(x: i32, y: i32, s: i32, m: i32, n: i32, max_move: i32, mem: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
            if s > max_move { return 0; }
            if x < 0 || y < 0 || x >= m || y >= n { return 1; }
            if let Some(&v) = mem.get(&(x, y, s)) {
                return v;
            }
            let mut ret = dfs(x + 1, y, s + 1, m, n, max_move, mem) as i64 +
                dfs(x - 1, y, s + 1, m, n, max_move, mem) as i64 +
                dfs(x, y + 1, s + 1, m, n, max_move, mem) as i64 +
                dfs(x, y - 1, s + 1, m, n, max_move, mem) as i64;
            ret = ret % 1000000007;
            let ret = ret as i32;
            mem.insert((x, y, s), ret);
            ret
        }

        dfs(start_row, start_column, 0, m, n, max_move, &mut HashMap::new())
    }
}