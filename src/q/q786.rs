use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // 看到数据量不大，就尝试暴力一下，结果直接AC了，赶时间，先Mark，后面再来看有什么其他方法优化
        // AC 188ms 19.3mb 59/59
        let n = arr.len();
        let mut ans = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                ans.push((arr[i] as f64 / arr[j] as f64, i, j));
            }
        }
        ans.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        vec![arr[ans[k as usize - 1].1], arr[ans[k as usize - 1].2]]
    }
}