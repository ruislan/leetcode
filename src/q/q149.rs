use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 两个点是直线可以通过斜率来判断，
        // 那么如果同时出现斜率最多的就是解
        // 这里要注意重复的点
        // AC 20ms 2mb
        let n = points.len();
        let mut answer = 1; // 如果只有一个点，就是1
        for i in 0..n {
            let mut map = std::collections::HashMap::new();
            let p1 = &points[i];
            for j in 0..n {
                let p2 = &points[j];
                if p1 == p2 { continue; } // 相同点，不计算
                let k =
                    if p1[0] == p2[0] { // 可能两点处于一条横直线上，就没有斜率
                        None
                    } else { // 计算两点的斜率
                        Some(((p2[1] - p1[1]) as f64 / (p2[0] - p1[0]) as f64).to_string())
                    };
                *map.entry(k).or_insert(0) += 1; // 相同斜率，增加1点
            }
            for &v in map.values() {
                answer = answer.max(v + 1); // 因为前面相同点没有计算，这里+1这个点
            }
        }
        answer
    }
}