use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        // 方法1
        // 暴力解决
        // 三层循环，直接找出符合条件的
        // Passed 8ms 2.2mb
        let n = arr.len();
        let mut answer = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if (arr[i] - arr[j]).abs() <= a && (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        answer += 1;
                    }
                }
            }
        }
        answer
    }
}