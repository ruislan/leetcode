use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 迭代queries，取出每对(L,R)，再取出arr中的值进行xor计算即可
        // Passed 628ms 3.9mb
        // let mut res = vec![0; queries.len()];
        // for i in 0..queries.len() {
        //     res[i] = arr[queries[i][0] as usize];
        //     for j in (queries[i][0] + 1)..=queries[i][1] {
        //         res[i] ^= arr[j as usize];
        //     }
        // }
        // res

        // 方法1优雅方式（一行）
        // Passed 356ms 3.9mb
        // queries
        //     .iter()
        //     .map(|query| ((query[0] + 1)..=query[1]).fold(arr[query[0] as usize], |acc, x| acc ^ arr[x as usize]))
        //     .collect()

        // 方法二
        // 求出arr的0-1，0-2，0-N的所有XOR的值
        // 迭代queries的每个q, 取出0..q[0]的XOR值 ^ 0..=q[1]即是q[0]..=q[1]的值
        // Passed 8ms 3.9mb (16ms 3.9mb)
        let mut arr = arr;
        (1..arr.len()).for_each(|i| arr[i] ^= arr[i - 1]);
        queries.iter().map(|q| if q[0] == 0 { arr[q[1] as usize] } else { arr[(q[0] - 1) as usize] ^ arr[q[1] as usize] }).collect()
    }
}

#[test]
fn test_q1310() {
    // assert_eq!(Solution::xor_queries(vec![], vec![vec![]]), vec![]);
    assert_eq!(Solution::xor_queries(vec![1, 3, 4, 8], vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]), vec![2, 7, 14, 8]);
}