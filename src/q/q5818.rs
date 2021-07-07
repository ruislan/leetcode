use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_commom_subsequence(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 一开始还以为是经典DP，然后发现是多个数组，
        // 最后发现数据范围是1-100，也就是O(n^2)才1万，这下就简单了，
        //  1. 我们定义一个一个101 * arrays.len()大小的矩阵，
        //     最后看是不是都是true
        // AC 4ms 2.3mb
        // let n = arrays.len();
        // let mut metrix = vec![vec![false; 101]; n];
        // for i in 0..n {
        //     for j in 0..arrays[i].len() {
        //         let index = arrays[i][j] as usize;
        //         metrix[i][index] = true;
        //     }
        // }
        // let mut answer = Vec::new();
        // for j in 0..=100 {
        //     if !(0..n).any(|i| !metrix[i][j]) {
        //         answer.push(j as i32);
        //     }
        // }
        // answer

        //  2. 我们定义一个101的数组，只要是那个数字的累加进去，最后看每个总是是不是n
        // AC 0ms 2.1mb
        let n = arrays.len();
        let mut freq = [0; 101];
        (0..n).for_each(|i| (0..arrays[i].len()).for_each(|j| freq[arrays[i][j] as usize] += 1));
        (0..101).filter(|&i| freq[i] as usize == n).map(|i| i as i32).collect()
    }
}