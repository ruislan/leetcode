use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        // 方法1：
        // 经过的每个点都存储到hashmap里面？如果再次经过就表示map中存在？
        // 变量：
        // 1. 当前点
        // 2. 当前方向
        // 3. 经过的点的hashset
        // 逻辑：
        // 迭代distance，然后处理起止位置上的每个点，如果存在点，就返回真
        // 复杂度： O(n^2)。10^5 * 10^5
        // 结论： 可能会超时
        // Not AC ，超时 28/29
        // use std::collections::HashSet;
        // #[derive(Clone, Copy)]
        // enum Direction { N, W, S, E }
        // let mut set = HashSet::new();
        // let mut cur = (0, 0);
        // set.insert(cur);
        // let mut direction = Direction::N;
        // for i in 0..distance.len() {
        //     let mut base = (0, 0);
        //     match direction {
        //         Direction::N => {
        //             base = (0, 1);
        //             direction = Direction::W;
        //         }
        //         Direction::W => {
        //             base = (-1, 0);
        //             direction = Direction::S;
        //         }
        //         Direction::S => {
        //             base = (0, -1);
        //             direction = Direction::E;
        //         }
        //         Direction::E => {
        //             base = (1, 0);
        //             direction = Direction::N;
        //         }
        //     };
        //     for _ in 0..distance[i] {
        //         cur.0 += base.0;
        //         cur.1 += base.1;
        //         if !set.insert(cur) {
        //             return true;
        //         }
        //     }
        // }
        // false

        // 方法2
        // 我们注意到它始终是逆时针走的，也就是说相交的情况是可以分情况处理的
        // 其实画画图，就能发现，只有三种情况是相交的，
        // 情况1，左穿右，如果i-1<=i-3，i>=i-2，就相交
        //     -----
        //     |   |
        //     |   |
        //     |---|--
        //         /
        // 情况2，下穿上，如果提前转向，那么一定会相交，如果刚好相等，也会相交，如果超过，则不会
        // 这里会有5条线 i..i-4。 如果i+(i-4) >= i-2并且 i-1 == i-3才可以
        //     (i-3)
        //     -----
        //     |   |
        //(i-2)|   | (i-4)
        //     |
        //     |---| (i)
        //     (i-1)
        // 情况3，右穿左，这里就是情况2不相交的情况，最后一次要相交的情况
        // 这里会有6条线 i..i-5。 可以看出来要相交那么 i + i-4 >= i-2 并且 i-1 + i-5 >= i-3 并且 i - 1 <= i-3 并且 i-2 > i-4
        //     (i-4)
        //     -----        i
        //     |   |      ------|
        //(i-3)|   | (i-5)      |(i-1)
        //     |                |
        //     |----------------
        //     (i-2)
        // 再往后，其实就会出现已经归纳的情况，这样所有的情况都照顾到了
        // AC 0ms 2.9mb
        // P.S 突然发现很像判断只能逆时针操作的贪吃蛇是否Game Over
        let n = distance.len();
        if n < 4 { return false; }
        for i in 3..n {
            if distance[i] >= distance[i - 2] && distance[i - 1] <= distance[i - 3] { return true; }
            if i >= 4 && distance[i - 1] == distance[i - 3] && distance[i] + distance[i - 4] >= distance[i - 2] { return true; }
            if i >= 5 && distance[i - 1] <= distance[i - 3] && distance[i - 2] > distance[i - 4] && distance[i] + distance[i - 4] >= distance[i - 2] && distance[i - 1] + distance[i - 5] >= distance[i - 3] { return true; }
        }
        false
    }
}