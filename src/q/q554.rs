use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 要想一条线拉下来穿过的砖块最少，必然是穿过的缝隙越多越好
        // 那么我们将砖块对齐，如果对齐的砖块越多，自然就是答案
        // 当然左右两头是不能算的，不然这样就没法对齐了
        // 用hashmap来记录每个位置的存在的砖块数量，高度就是所有的砖块行数
        // AC 4ms 2.9mb
        let mut bricks = std::collections::HashMap::new();
        for i in 0..wall.len() {
            let mut sum = 0;
            for j in 0..wall[i].len() {
                sum += wall[i][j];
                if j != wall[i].len() - 1 {
                    *bricks.entry(sum).or_insert(0) += 1;
                }
            }
        }
        wall.len() as i32 - bricks.values().max().unwrap()
    }
}