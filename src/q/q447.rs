use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 三重循环分别求出每个点距离另外两个点的距离
        // O(n^3)
        // 超时
        // let n = points.len();
        // let mut ans = 0;
        // for i in 0..n {
        //     for j in 0..n {
        //         if i == j { continue; }
        //         for k in 0..n {
        //             if i == k || j == k { continue; }
        //             let i_j = (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);
        //             let i_k = (points[i][0] - points[k][0]).pow(2) + (points[i][1] - points[k][1]).pow(2);
        //
        //             if i_j == i_k {
        //                 ans += 1;
        //             }
        //         }
        //     }
        // }
        // ans

        // 方法2
        // 两重循环
        // 首先求出某一点到其他各点的距离，分组存储起来
        // 例如：
        // a b c
        // a: (b, 1),(c,2)  b: (a,1)/(c,1)
        // 这样只需要遍历所有的点，每个点里面的如果存在值相等的就求出两两全排列数（大于2的），全排列公式
        // O(n^2)
        // AC 88ms 6.8mb 32/32
        let n = points.len();
        let mut mem = vec![std::collections::HashMap::new(); n];
        for i in 0..n {
            for j in i + 1..n {
                let distance = (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);
                *mem[i].entry(distance).or_insert(0) += 1;
                *mem[j].entry(distance).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for &x in mem[i].values() {
                if x > 1 {
                    ans += x * (x - 1);
                }
            }
        }
        ans
    }
}