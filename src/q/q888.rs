use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 求出a，b两边各自的和，a，b的平均数avg
        // 迭代a，当a少掉一个数之后，它需要增加一个数成为平均值
        // 而增加的这个数如果在b中能够找到，那么就是要更换的
        // 我们将b的糖果大小放入map中方便查询
        // AC 12ms 2.2mb
        let sum_a: i32 = a.iter().sum();
        let sum_b: i32 = b.iter().sum();
        let avg = (sum_a + sum_b) >> 1;
        let mut set_b = std::collections::HashSet::new();
        for x in b { set_b.insert(x); }
        for x in a {
            let y = avg - (sum_a - x);
            if set_b.contains(&y) { return vec![x, y]; }
        }
        vec![]
    }
}