use crate::q::Solution;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        // 方法1
        // 直接排序之后，去掉头尾的5%个数据，求和并除以数量即可
        // Passed 0ms 2.2mb
        let mut arr = arr;
        arr.sort_unstable();
        let i = (arr.len() as f32 * 0.05_f32) as usize;
        (i..arr.len() - i).map(|i| arr[i]).sum::<i32>() as f64 / (arr.len() - i * 2) as f64
    }
}