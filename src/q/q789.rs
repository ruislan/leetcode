use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        // 方法1
        // ghosts如果比吃豆人先到target，那么必然能够吃到豆豆，他可以不动，等着
        // 所以如果ghosts[i]的到达target的步数小于或者等于(0,0)到达target的步数，那么就为false
        // 所以问题就转化成了比较曼哈顿距离最短
        // AC 0ms 2mb
        let d = target[0].abs() + target[1].abs();
        for v in ghosts.iter() {
            if (v[0] - target[0]).abs() + (v[1] - target[1]).abs() <= d {
                return false;
            }
        }
        true
    }
}