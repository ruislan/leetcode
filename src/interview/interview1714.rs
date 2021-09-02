use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // 直接排序，然后取前k个
        // AC 8ms 2.9mb 29/29
        let mut arr = arr;
        arr.sort_unstable();
        let mut ans = Vec::new();
        let k = k as usize;
        for i in 0..arr.len() {
            if i == k { break; }
            ans.push(arr[i]);
        }
        ans
    }
}