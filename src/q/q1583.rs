use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 循环比对paris[i]与Paris[j]
        // 分别令x = Paris[i][0]                         ｜ x = paris[i][1]
        //     u = paris[j][0] | u = paris[j][1]          u = paris[j][0] | u = paris[j][1]
        // 满足条件的x都是不开心的
        // AC 156ms 3.7mb
        // P.S 不是最佳解，有时间再优化了
        fn is_unhappy(x: i32, y: i32, u: i32, v: i32, preferences: &Vec<Vec<i32>>, n: usize) -> bool {
            let (mut pos_x, mut pos_y, mut pos_u, mut pos_v) = (0, 0, 0, 0);
            for i in 0..(n - 1) {
                if preferences[x as usize][i] == y {
                    pos_y = i;
                }
                if preferences[x as usize][i] == u {
                    pos_u = i;
                }
                if preferences[u as usize][i] == x {
                    pos_x = i;
                }
                if preferences[u as usize][i] == v {
                    pos_v = i;
                }
            }
            pos_u < pos_y && pos_x < pos_v
        }

        let n = n as usize;
        let mut answer = std::collections::HashSet::new();
        let m = n / 2;
        for i in 0..m {
            for j in i + 1..m {
                let p1 = &pairs[i];
                let p2 = &pairs[j];
                if is_unhappy(p1[0], p1[1], p2[0], p2[1], &preferences, n)
                    || is_unhappy(p1[0], p1[1], p2[1], p2[0], &preferences, n) {
                    answer.insert(p1[0]);
                }
                if is_unhappy(p1[1], p1[0], p2[0], p2[1], &preferences, n)
                    || is_unhappy(p1[1], p1[0], p2[1], p2[0], &preferences, n) {
                    answer.insert(p1[1]);
                }
                if is_unhappy(p2[0], p2[1], p1[0], p1[1], &preferences, n)
                    || is_unhappy(p2[0], p2[1], p1[1], p1[0], &preferences, n) {
                    answer.insert(p2[0]);
                }
                if is_unhappy(p2[1], p2[0], p1[0], p1[1], &preferences, n)
                    || is_unhappy(p2[1], p2[0], p1[1], p1[0], &preferences, n) {
                    answer.insert(p2[1]);
                }
            }
        }
        answer.len() as i32
    }
}