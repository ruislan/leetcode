use crate::Solution;

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        // 方法1
        // n代表位数，用公式10.pow(n + 1) - 1求到最大的n位数记为max
        // 例如n = 4, 10.pow(5)= 1万，减去1为9999
        // 然后迭代1..=max，并map每个数字，然后收集为vec即可
        vec![]
    }
}