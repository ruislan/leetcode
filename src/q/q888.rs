use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 求出a，b各自的和，然后求出平均数fair
        // 我们将b的糖果大小放入map中方便查询
        // 因为只需要交换一次，所以我们只需要迭代a，
        // 然后判断对应的数字是否在map中即可
        // 对应的数字公式为y = fair - (sum_a - a[i])
        // Passed 20ms 2.4mb
        let sum_a:i32 = a.iter().sum();
        let sum_b:i32 = b.iter().sum();
        let fair = (sum_a + sum_b) >> 1;
        let mut map_b = std::collections::HashMap::new();
        for x in b {
            *map_b.entry(x).or_insert(0) += 1;
        }
        for x in a {
            let y = fair - sum_a + x;
            if map_b.get(&y).is_some() {
                return vec![x, y];
            }
        }
        vec![]
    }
}